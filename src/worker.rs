//! Background worker for processing queued jobs
//!
//! This module implements the worker that:
//! - Performs atomic renames for same-filesystem moves
//! - Detects cross-device moves via statfs
//! - Performs chunked streaming copy + fsync + unlink for cross-device moves
//! - Implements exponential backoff retry for failed jobs
//! - Integrates with the trash manager for rm jobs

use anyhow::{Context, Result};
use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write};
use std::os::unix::fs::MetadataExt;
use std::path::Path;
use tracing::{info, warn, error, debug};
use uuid::Uuid;

use crate::queue::{Job, JobQueue, OperationType};
use crate::trash::TrashManager;
use crate::config::Config;

/// Default buffer size for chunked copy (64KB)
const DEFAULT_BUFFER_SIZE: usize = 64 * 1024;

/// Maximum retry count for failed jobs
const MAX_RETRIES: i32 = 5;

/// Base delay for exponential backoff (in milliseconds)
const BASE_RETRY_DELAY_MS: u64 = 100;

/// Worker that processes queued jobs
pub struct Worker {
    queue: JobQueue,
    trash_manager: Option<TrashManager>,
    config: Config,
    buffer_size: usize,
}

impl Worker {
    /// Create a new worker instance
    ///
    /// # Arguments
    /// * `queue` - The job queue to process
    /// * `trash_manager` - Optional trash manager for rm jobs
    /// * `config` - Configuration for space guard settings
    /// * `buffer_size` - Buffer size for chunked copy (default: 64KB)
    pub fn new(queue: JobQueue, trash_manager: Option<TrashManager>, config: Config, buffer_size: Option<usize>) -> Self {
        Worker {
            queue,
            trash_manager,
            config,
            buffer_size: buffer_size.unwrap_or(DEFAULT_BUFFER_SIZE),
        }
    }

    /// Process a single job
    ///
    /// This is the main entry point for job processing. It:
    /// 1. Determines the operation type (move, copy, delete)
    /// 2. Detects if source and destination are on the same filesystem
    /// 3. Executes the appropriate operation (atomic rename or copy+fsync+unlink)
    /// 4. Marks the job as done or failed in the queue
    ///
    /// # Arguments
    /// * `job` - The job to process
    ///
    /// # Returns
    /// Ok(()) if the job was processed successfully, Err otherwise
    pub fn process_job(&self, job: &Job) -> Result<()> {
        info!("Processing job {}: {:?} from {} to {:?}",
              job.uuid, job.op_type, job.source.display(),
              job.dest.as_ref().map(|p| p.display().to_string()).unwrap_or_else(|| "N/A".to_string()));

        let result = match job.op_type {
            OperationType::Move => {
                if let Some(ref dest) = job.dest {
                    self.process_move(&job.source, dest)
                } else {
                    Err(anyhow::anyhow!("Move job missing destination"))
                }
            }
            OperationType::Copy => {
                if let Some(ref dest) = job.dest {
                    self.process_copy(&job.source, dest)
                } else {
                    Err(anyhow::anyhow!("Copy job missing destination"))
                }
            }
            OperationType::Delete => {
                self.process_delete(&job.source)
            }
        };

        match result {
            Ok(()) => {
                self.queue.mark_done(&job.uuid)
                    .context("Failed to mark job as done")?;
                info!("Job {} completed successfully", job.uuid);
            }
            Err(e) => {
                self.queue.mark_failed(&job.uuid)
                    .context("Failed to mark job as failed")?;
                error!("Job {} failed: {}", job.uuid, e);
                return Err(e);
            }
        }

        Ok(())
    }

    /// Process a move operation
    ///
    /// Detects if source and destination are on the same filesystem:
    /// - Same filesystem: atomic rename
    /// - Cross-device: copy + fsync + unlink
    fn process_move(&self, source: &Path, dest: &Path) -> Result<()> {
        let same_fs = self.same_filesystem(source, dest)?;

        if same_fs {
            debug!("Same filesystem detected, using atomic rename");
            self.atomic_rename(source, dest)?;
        } else {
            debug!("Cross-device move detected, using copy + fsync + unlink");
            self.copy_with_fsync(source, dest)?;
            fs::remove_file(source)
                .context("Failed to remove source after cross-device copy")?;
        }

        Ok(())
    }

    /// Process a copy operation
    ///
    /// Always uses copy + fsync (even on same filesystem for consistency)
    fn process_copy(&self, source: &Path, dest: &Path) -> Result<()> {
        self.copy_with_fsync(source, dest)?;
        Ok(())
    }

    /// Process a delete operation (move to trash)
    ///
    /// Integrates with the trash manager if available
    /// Handles disk space guard and auto-culling
    fn process_delete(&self, source: &Path) -> Result<()> {
        if let Some(ref trash_manager) = self.trash_manager {
            let trash_path = trash_manager.compute_trash_path(source, 1);
            
            // Try to move to trash (this will check disk space and cull if needed)
            if let Err(e) = trash_manager.move_to_trash(source, &trash_path, "rm", &self.config) {
                // Check if this is the special error indicating trash is full
                if e.to_string() == "TRASH_FULL_DELETE_DIRECT" {
                    warn!("Trash is full, deleting directly instead of trashing");
                    fs::remove_file(source)
                        .context("Failed to delete file directly")?;
                } else {
                    return Err(e);
                }
            }
        } else {
            // Fallback: direct delete if no trash manager
            fs::remove_file(source)
                .context("Failed to delete file")?;
        }
        Ok(())
    }

    /// Check if two paths are on the same filesystem
    ///
    /// Uses device IDs (stat) to compare filesystems
    ///
    /// # Arguments
    /// * `path1` - First path
    /// * `path2` - Second path
    ///
    /// # Returns
    /// true if both paths are on the same filesystem, false otherwise
    fn same_filesystem(&self, path1: &Path, path2: &Path) -> Result<bool> {
        // If file doesn't exist, check parent directory instead
        let check_path1 = if path1.exists() {
            path1.to_path_buf()
        } else {
            path1.parent()
                .ok_or_else(|| anyhow::anyhow!("Path has no parent directory"))?
                .to_path_buf()
        };

        let check_path2 = if path2.exists() {
            path2.to_path_buf()
        } else {
            path2.parent()
                .ok_or_else(|| anyhow::anyhow!("Path has no parent directory"))?
                .to_path_buf()
        };

        let metadata1 = std::fs::metadata(&check_path1)
            .context("Failed to get metadata for first path")?;
        let metadata2 = std::fs::metadata(&check_path2)
            .context("Failed to get metadata for second path")?;

        // Compare device IDs to determine if on same filesystem
        let dev1 = metadata1.dev();
        let dev2 = metadata2.dev();

        Ok(dev1 == dev2)
    }

    /// Perform atomic rename for same-filesystem moves
    ///
    /// Uses std::fs::rename which is atomic on the same filesystem
    ///
    /// # Arguments
    /// * `source` - Source path
    /// * `dest` - Destination path
    fn atomic_rename(&self, source: &Path, dest: &Path) -> Result<()> {
        // Ensure destination parent directory exists
        if let Some(parent) = dest.parent() {
            fs::create_dir_all(parent)
                .context("Failed to create destination parent directory")?;
        }

        fs::rename(source, dest)
            .with_context(|| format!(
                "Failed to rename {} to {}",
                source.display(),
                dest.display()
            ))?;

        debug!("Atomic rename succeeded: {} -> {}", source.display(), dest.display());
        Ok(())
    }

    /// Perform chunked streaming copy with fsync
    ///
    /// This is used for cross-device moves and copy operations:
    /// 1. Copy to temporary file
    /// 2. fsync the file
    /// 3. fsync the containing directory
    /// 4. Atomic rename to final destination
    ///
    /// # Arguments
    /// * `source` - Source path
    /// * `dest` - Destination path
    fn copy_with_fsync(&self, source: &Path, dest: &Path) -> Result<()> {
        // Create temporary file in the same directory as destination
        let dest_dir = dest.parent()
            .ok_or_else(|| anyhow::anyhow!("Destination has no parent directory"))?;
        fs::create_dir_all(dest_dir)
            .context("Failed to create destination directory")?;

        let temp_path = dest_dir.join(format!(".tmp_{}", Uuid::new_v4()));

        // Open source and destination files
        let mut src_file = File::open(source)
            .context("Failed to open source file")?;
        let mut dest_file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&temp_path)
            .context("Failed to create temporary destination file")?;

        // Copy in chunks to avoid loading entire file into memory
        let mut buffer = vec![0u8; self.buffer_size];
        loop {
            let bytes_read = src_file.read(&mut buffer)
                .context("Failed to read from source file")?;
            if bytes_read == 0 {
                break;
            }
            dest_file.write_all(&buffer[..bytes_read])
                .context("Failed to write to destination file")?;
        }

        // Sync the file to disk
        dest_file.sync_all()
            .context("Failed to fsync destination file")?;

        // Sync the directory to ensure the directory entry is persistent
        if let Some(dir) = dest_dir.to_str() {
            let dir_file = File::open(dir)
                .context("Failed to open destination directory for fsync")?;
            dir_file.sync_all()
                .context("Failed to fsync destination directory")?;
        }

        // Atomic rename from temp to final destination
        fs::rename(&temp_path, dest)
            .with_context(|| format!(
                "Failed to rename temporary file {} to {}",
                temp_path.display(),
                dest.display()
            ))?;

        debug!("Copy with fsync succeeded: {} -> {}", source.display(), dest.display());
        Ok(())
    }

    /// Process jobs from the queue with retry logic
    ///
    /// This is the main worker loop:
    /// 1. Dequeue the next pending job
    /// 2. Process the job
    /// 3. If failed and retry count < max, requeue with exponential backoff
    /// 4. Repeat until queue is empty or shutdown requested
    ///
    /// # Arguments
    /// * `shutdown_check` - Optional callback to check for shutdown request
    pub fn process_queue<F>(&self, shutdown_check: Option<&F>) -> Result<()>
    where
        F: Fn() -> bool,
    {
        loop {
            // Check for shutdown
            if let Some(check) = shutdown_check {
                if check() {
                    info!("Shutdown requested, worker exiting");
                    break;
                }
            }

            // Dequeue next job
            let job = match self.queue.dequeue()? {
                Some(job) => job,
                None => {
                    // No more jobs, exit
                    debug!("Queue is empty, worker exiting");
                    break;
                }
            };

            // Process the job
            let result = self.process_job(&job);

            // If failed and retries remaining, requeue with backoff
            if result.is_err() && job.retry_count < MAX_RETRIES {
                let delay_ms = BASE_RETRY_DELAY_MS * 2_u64.pow(job.retry_count as u32);
                warn!(
                    "Job {} failed (attempt {}/{}), retrying in {}ms",
                    job.uuid,
                    job.retry_count + 1,
                    MAX_RETRIES,
                    delay_ms
                );
                std::thread::sleep(std::time::Duration::from_millis(delay_ms));

                // Reset job to queued for retry
                // Note: mark_failed already incremented retry_count
                self.queue.reset_to_queued(&job.uuid)
                    .context("Failed to reset job to queued for retry")?;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use std::fs;
    use crate::config::Config;
    use crate::queue::JobStatus;

    fn create_test_worker() -> (Worker, TempDir) {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("test_queue.db");
        let queue = JobQueue::new(&db_path).unwrap();

        let mut config = Config::default();
        let trash_root = temp_dir.path().join("trash");
        fs::create_dir_all(&trash_root).unwrap();
        config.trash.root = trash_root.clone();
        let trash_manager = Some(TrashManager::new(&config));

        let worker = Worker::new(queue, trash_manager, config, Some(4096));
        (worker, temp_dir)
    }

    #[test]
    fn test_same_filesystem_detection() {
        let temp_dir = TempDir::new().unwrap();
        let file1 = temp_dir.path().join("file1.txt");
        let file2 = temp_dir.path().join("file2.txt");

        fs::write(&file1, "test").unwrap();

        let db_path = temp_dir.path().join("test_queue.db");
        let queue = JobQueue::new(&db_path).unwrap();
        let config = Config::default();
        let worker = Worker::new(queue, None, config, None);

        // Files in same directory should be on same filesystem
        let result = worker.same_filesystem(&file1, &file2);
        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[test]
    fn test_atomic_rename() {
        let temp_dir = TempDir::new().unwrap();
        let source = temp_dir.path().join("source.txt");
        let dest = temp_dir.path().join("dest.txt");

        fs::write(&source, "test content").unwrap();

        let db_path = temp_dir.path().join("test_queue.db");
        let queue = JobQueue::new(&db_path).unwrap();
        let config = Config::default();
        let worker = Worker::new(queue, None, config, None);

        worker.atomic_rename(&source, &dest).unwrap();

        assert!(!source.exists());
        assert!(dest.exists());
        assert_eq!(fs::read_to_string(&dest).unwrap(), "test content");
    }

    #[test]
    fn test_copy_with_fsync() {
        let temp_dir = TempDir::new().unwrap();
        let source = temp_dir.path().join("source.txt");
        let dest = temp_dir.path().join("dest.txt");

        fs::write(&source, "test content").unwrap();

        let db_path = temp_dir.path().join("test_queue.db");
        let queue = JobQueue::new(&db_path).unwrap();
        let config = Config::default();
        let worker = Worker::new(queue, None, config, None);

        worker.copy_with_fsync(&source, &dest).unwrap();

        // Source should still exist
        assert!(source.exists());
        // Destination should exist with same content
        assert!(dest.exists());
        assert_eq!(fs::read_to_string(&dest).unwrap(), "test content");
    }

    #[test]
    fn test_copy_with_fsync_creates_parent_dirs() {
        let temp_dir = TempDir::new().unwrap();
        let source = temp_dir.path().join("source.txt");
        let dest = temp_dir.path().join("nested/deep/dest.txt");

        fs::write(&source, "test content").unwrap();

        let db_path = temp_dir.path().join("test_queue.db");
        let queue = JobQueue::new(&db_path).unwrap();
        let config = Config::default();
        let worker = Worker::new(queue, None, config, None);

        worker.copy_with_fsync(&source, &dest).unwrap();

        assert!(dest.exists());
        assert_eq!(fs::read_to_string(&dest).unwrap(), "test content");
    }

    #[test]
    fn test_process_move_same_fs() {
        let (worker, temp_dir) = create_test_worker();
        let source = temp_dir.path().join("source.txt");
        let dest = temp_dir.path().join("dest.txt");

        fs::write(&source, "test content").unwrap();

        let job = Job {
            uuid: Uuid::new_v4(),
            source: source.clone(),
            dest: Some(dest.clone()),
            status: JobStatus::Queued,
            retry_count: 0,
            op_type: OperationType::Move,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };

        worker.process_job(&job).unwrap();

        assert!(!source.exists());
        assert!(dest.exists());
    }

    #[test]
    fn test_process_copy() {
        let (worker, temp_dir) = create_test_worker();
        let source = temp_dir.path().join("source.txt");
        let dest = temp_dir.path().join("dest.txt");

        fs::write(&source, "test content").unwrap();

        let job = Job {
            uuid: Uuid::new_v4(),
            source: source.clone(),
            dest: Some(dest.clone()),
            status: JobStatus::Queued,
            retry_count: 0,
            op_type: OperationType::Copy,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };

        worker.process_job(&job).unwrap();

        assert!(source.exists());
        assert!(dest.exists());
    }

    #[test]
    fn test_process_delete_with_trash() {
        let (worker, temp_dir) = create_test_worker();
        let source = temp_dir.path().join("source.txt");

        fs::write(&source, "test content").unwrap();

        let job = Job {
            uuid: Uuid::new_v4(),
            source: source.clone(),
            dest: None,
            status: JobStatus::Queued,
            retry_count: 0,
            op_type: OperationType::Delete,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };

        worker.process_job(&job).unwrap();

        // Source should be moved to trash
        assert!(!source.exists());
    }

    #[test]
    fn test_process_move_marks_job_done() {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("test_queue.db");
        let queue = JobQueue::new(&db_path).unwrap();

        let mut config = Config::default();
        let trash_root = temp_dir.path().join("trash");
        fs::create_dir_all(&trash_root).unwrap();
        config.trash.root = trash_root.clone();
        let trash_manager = Some(TrashManager::new(&config));

        let source = temp_dir.path().join("source.txt");
        let dest = temp_dir.path().join("dest.txt");

        fs::write(&source, "test content").unwrap();

        let job = Job {
            uuid: Uuid::new_v4(),
            source: source.clone(),
            dest: Some(dest.clone()),
            status: JobStatus::Queued,
            retry_count: 0,
            op_type: OperationType::Move,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };

        queue.enqueue(&job).unwrap();
        
        let worker = Worker::new(queue, trash_manager, config, Some(4096));
        worker.process_job(&job).unwrap();

        let status = worker.queue.get_status(&job.uuid).unwrap();
        assert_eq!(status, Some(JobStatus::Done));
    }

    #[test]
    fn test_process_move_marks_job_failed_on_error() {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("test_queue.db");
        let queue = JobQueue::new(&db_path).unwrap();

        let mut config = Config::default();
        let trash_root = temp_dir.path().join("trash");
        fs::create_dir_all(&trash_root).unwrap();
        config.trash.root = trash_root.clone();
        let trash_manager = Some(TrashManager::new(&config));

        let source = temp_dir.path().join("nonexistent.txt");
        let dest = temp_dir.path().join("dest.txt");

        let job = Job {
            uuid: Uuid::new_v4(),
            source: source.clone(),
            dest: Some(dest.clone()),
            status: JobStatus::Queued,
            retry_count: 0,
            op_type: OperationType::Move,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };

        queue.enqueue(&job).unwrap();
        
        let worker = Worker::new(queue, trash_manager, config, Some(4096));
        let result = worker.process_job(&job);

        assert!(result.is_err());

        let status = worker.queue.get_status(&job.uuid).unwrap();
        assert_eq!(status, Some(JobStatus::Failed));
    }

    #[test]
    fn test_buffer_size_configuration() {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("test_queue.db");
        let queue = JobQueue::new(&db_path).unwrap();
        let config = Config::default();

        let worker = Worker::new(queue, None, config, Some(8192));
        assert_eq!(worker.buffer_size, 8192);
    }

    #[test]
    fn test_default_buffer_size() {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("test_queue.db");
        let queue = JobQueue::new(&db_path).unwrap();
        let config = Config::default();

        let worker = Worker::new(queue, None, config, None);
        assert_eq!(worker.buffer_size, DEFAULT_BUFFER_SIZE);
    }
}
