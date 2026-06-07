use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use rusqlite::{params, Connection, OptionalExtension};
use std::path::PathBuf;
use tracing::{debug, trace};
use uuid::Uuid;

/// Job operation types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OperationType {
    Move,
    Copy,
    Delete,
}

impl OperationType {
    fn from_i32(value: i32) -> Result<Self> {
        match value {
            0 => Ok(OperationType::Move),
            1 => Ok(OperationType::Copy),
            2 => Ok(OperationType::Delete),
            _ => anyhow::bail!("Invalid operation type: {}", value),
        }
    }

    fn to_i32(self) -> i32 {
        match self {
            OperationType::Move => 0,
            OperationType::Copy => 1,
            OperationType::Delete => 2,
        }
    }
}

/// Job status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JobStatus {
    Queued,
    Running,
    Done,
    Failed,
}

impl JobStatus {
    fn from_i32(value: i32) -> Result<Self> {
        match value {
            0 => Ok(JobStatus::Queued),
            1 => Ok(JobStatus::Running),
            2 => Ok(JobStatus::Done),
            3 => Ok(JobStatus::Failed),
            _ => anyhow::bail!("Invalid job status: {}", value),
        }
    }

    fn to_i32(self) -> i32 {
        match self {
            JobStatus::Queued => 0,
            JobStatus::Running => 1,
            JobStatus::Done => 2,
            JobStatus::Failed => 3,
        }
    }
}

/// Job record in the queue
#[derive(Debug, Clone)]
pub struct Job {
    pub uuid: Uuid,
    pub source: PathBuf,
    pub dest: Option<PathBuf>,
    pub status: JobStatus,
    pub retry_count: i32,
    pub op_type: OperationType,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Job queue manager
pub struct JobQueue {
    conn: Connection,
}

impl JobQueue {
    /// Initialize the job queue with WAL mode
    pub fn new(db_path: &PathBuf) -> Result<Self> {
        let conn = Connection::open(db_path)
            .context("Failed to open job queue database")?;

        // Enable WAL mode for concurrent read/write safety
        // Use query_one instead of execute because PRAGMA returns results
        let _wal_mode: String = conn.query_row("PRAGMA journal_mode=WAL", [], |row| row.get(0))
            .context("Failed to enable WAL mode")?;

        // Create the jobs table if it doesn't exist
        conn.execute(
            "CREATE TABLE IF NOT EXISTS jobs (
                uuid TEXT PRIMARY KEY,
                source TEXT NOT NULL,
                dest TEXT,
                status INTEGER NOT NULL,
                retry_count INTEGER NOT NULL DEFAULT 0,
                op_type INTEGER NOT NULL,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            )",
            [],
        ).context("Failed to create jobs table")?;

        // Create index on status for efficient querying
        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_jobs_status ON jobs(status)",
            [],
        ).context("Failed to create status index")?;

        // Create index on created_at for crash recovery ordering
        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_jobs_created_at ON jobs(created_at)",
            [],
        ).context("Failed to create created_at index")?;

        debug!("Job queue initialized with WAL mode at {:?}", db_path);

        Ok(JobQueue { conn })
    }

    /// Enqueue a new job
    pub fn enqueue(&self, job: &Job) -> Result<()> {
        let now = Utc::now();
        self.conn.execute(
            "INSERT INTO jobs (uuid, source, dest, status, retry_count, op_type, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            params![
                job.uuid.to_string(),
                job.source.to_string_lossy().as_ref(),
                job.dest.as_ref().map(|p| p.to_string_lossy().to_string()),
                job.status.to_i32(),
                job.retry_count,
                job.op_type.to_i32(),
                job.created_at.to_rfc3339(),
                now.to_rfc3339(),
            ],
        ).context("Failed to enqueue job")?;

        trace!("Enqueued job {} with status {:?}", job.uuid, job.status);
        Ok(())
    }

    /// Dequeue the next pending job
    pub fn dequeue(&self) -> Result<Option<Job>> {
        let tx = self.conn.unchecked_transaction()
            .context("Failed to start transaction")?;

        // Find the oldest queued job
        let job = {
            let mut stmt = tx.prepare(
                "SELECT uuid, source, dest, status, retry_count, op_type, created_at, updated_at
                 FROM jobs
                 WHERE status = ?1
                 ORDER BY created_at ASC
                 LIMIT 1"
            ).context("Failed to prepare dequeue query")?;

            stmt.query_row(params![JobStatus::Queued.to_i32()], |row| {
                let uuid_str: String = row.get(0)?;
                let uuid = Uuid::parse_str(&uuid_str)
                    .map_err(|_| rusqlite::Error::InvalidQuery)?;

                let source_str: String = row.get(1)?;
                let dest_str: Option<String> = row.get(2)?;

                let status_i32: i32 = row.get(3)?;
                let status = JobStatus::from_i32(status_i32)
                    .map_err(|_| rusqlite::Error::InvalidQuery)?;

                let retry_count: i32 = row.get(4)?;

                let op_type_i32: i32 = row.get(5)?;
                let op_type = OperationType::from_i32(op_type_i32)
                    .map_err(|_| rusqlite::Error::InvalidQuery)?;

                let created_at_str: String = row.get(6)?;
                let created_at = DateTime::parse_from_rfc3339(&created_at_str)
                    .map_err(|_| rusqlite::Error::InvalidQuery)?
                    .with_timezone(&Utc);

                let updated_at_str: String = row.get(7)?;
                let updated_at = DateTime::parse_from_rfc3339(&updated_at_str)
                    .map_err(|_| rusqlite::Error::InvalidQuery)?
                    .with_timezone(&Utc);

                Ok(Job {
                    uuid,
                    source: PathBuf::from(source_str),
                    dest: dest_str.map(PathBuf::from),
                    status,
                    retry_count,
                    op_type,
                    created_at,
                    updated_at,
                })
            }).optional().context("Failed to query queued job")?
        };

        if let Some(ref job) = job {
            // Mark the job as running
            tx.execute(
                "UPDATE jobs SET status = ?1, updated_at = ?2 WHERE uuid = ?3",
                params![
                    JobStatus::Running.to_i32(),
                    Utc::now().to_rfc3339(),
                    job.uuid.to_string(),
                ],
            ).context("Failed to mark job as running")?;

            tx.commit().context("Failed to commit transaction")?;
            trace!("Dequeued job {} and marked as running", job.uuid);
        }

        Ok(job)
    }

    /// Mark a job as done
    pub fn mark_done(&self, uuid: &Uuid) -> Result<()> {
        let now = Utc::now();
        self.conn.execute(
            "UPDATE jobs SET status = ?1, updated_at = ?2 WHERE uuid = ?3",
            params![
                JobStatus::Done.to_i32(),
                now.to_rfc3339(),
                uuid.to_string(),
            ],
        ).context("Failed to mark job as done")?;

        trace!("Marked job {} as done", uuid);
        Ok(())
    }

    /// Mark a job as failed and increment retry count
    pub fn mark_failed(&self, uuid: &Uuid) -> Result<()> {
        let now = Utc::now();
        self.conn.execute(
            "UPDATE jobs SET status = ?1, retry_count = retry_count + 1, updated_at = ?2 WHERE uuid = ?3",
            params![
                JobStatus::Failed.to_i32(),
                now.to_rfc3339(),
                uuid.to_string(),
            ],
        ).context("Failed to mark job as failed")?;

        trace!("Marked job {} as failed", uuid);
        Ok(())
    }

    /// Get the status of a job
    pub fn get_status(&self, uuid: &Uuid) -> Result<Option<JobStatus>> {
        let mut stmt = self.conn.prepare(
            "SELECT status FROM jobs WHERE uuid = ?1"
        ).context("Failed to prepare status query")?;

        let status = stmt.query_row(params![uuid.to_string()], |row| {
            let status_i32: i32 = row.get(0)?;
            JobStatus::from_i32(status_i32)
                .map_err(|_| rusqlite::Error::InvalidQuery)
        }).optional().context("Failed to query job status")?;

        Ok(status)
    }

    /// Perform crash recovery: restart in-flight jobs or mark them as failed
    pub fn crash_recovery(&self, max_retries: i32) -> Result<()> {
        let now = Utc::now();

        // Find all running jobs
        let mut stmt = self.conn.prepare(
            "SELECT uuid, retry_count FROM jobs WHERE status = ?1"
        ).context("Failed to prepare crash recovery query")?;

        let job_rows = stmt.query_map(params![JobStatus::Running.to_i32()], |row| {
            let uuid_str: String = row.get(0)?;
            let uuid = Uuid::parse_str(&uuid_str)
                .map_err(|_| rusqlite::Error::InvalidQuery)?;
            let retry_count: i32 = row.get(1)?;
            Ok((uuid, retry_count))
        }).context("Failed to query running jobs")?;

        let mut recovered = 0;
        let mut failed = 0;

        for job_result in job_rows {
            let (uuid, retry_count) = job_result.context("Failed to process job row")?;

            if retry_count >= max_retries {
                // Mark as failed if max retries exceeded
                self.conn.execute(
                    "UPDATE jobs SET status = ?1, updated_at = ?2 WHERE uuid = ?3",
                    params![
                        JobStatus::Failed.to_i32(),
                        now.to_rfc3339(),
                        uuid.to_string(),
                    ],
                ).context("Failed to mark job as failed during recovery")?;
                failed += 1;
            } else {
                // Reset to queued for retry
                self.conn.execute(
                    "UPDATE jobs SET status = ?1, updated_at = ?2 WHERE uuid = ?3",
                    params![
                        JobStatus::Queued.to_i32(),
                        now.to_rfc3339(),
                        uuid.to_string(),
                    ],
                ).context("Failed to reset job to queued during recovery")?;
                recovered += 1;
            }
        }

        debug!(
            "Crash recovery completed: {} jobs reset to queued, {} jobs marked as failed",
            recovered, failed
        );

        Ok(())
    }

    /// Get the current queue depth (number of queued jobs)
    pub fn queue_depth(&self) -> Result<i64> {
        let depth: i64 = self.conn.query_row(
            "SELECT COUNT(*) FROM jobs WHERE status = ?1",
            params![JobStatus::Queued.to_i32()],
            |row| row.get(0),
        ).context("Failed to query queue depth")?;

        Ok(depth)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    fn create_test_queue() -> JobQueue {
        let temp_file = NamedTempFile::new().unwrap();
        JobQueue::new(&temp_file.path().to_path_buf()).unwrap()
    }

    #[test]
    fn test_queue_initialization() {
        let temp_file = NamedTempFile::new().unwrap();
        let queue = JobQueue::new(&temp_file.path().to_path_buf());
        assert!(queue.is_ok());
    }

    #[test]
    fn test_enqueue_and_dequeue() {
        let queue = create_test_queue();
        let job = Job {
            uuid: Uuid::new_v4(),
            source: PathBuf::from("/tmp/test.txt"),
            dest: Some(PathBuf::from("/tmp/test_moved.txt")),
            status: JobStatus::Queued,
            retry_count: 0,
            op_type: OperationType::Move,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        queue.enqueue(&job).unwrap();
        let dequeued = queue.dequeue().unwrap();
        assert!(dequeued.is_some());
        assert_eq!(dequeued.unwrap().uuid, job.uuid);
    }

    #[test]
    fn test_mark_done() {
        let queue = create_test_queue();
        let job = Job {
            uuid: Uuid::new_v4(),
            source: PathBuf::from("/tmp/test.txt"),
            dest: Some(PathBuf::from("/tmp/test_moved.txt")),
            status: JobStatus::Queued,
            retry_count: 0,
            op_type: OperationType::Move,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        queue.enqueue(&job).unwrap();
        queue.mark_done(&job.uuid).unwrap();

        let status = queue.get_status(&job.uuid).unwrap();
        assert_eq!(status, Some(JobStatus::Done));
    }

    #[test]
    fn test_mark_failed_increments_retry() {
        let queue = create_test_queue();
        let job = Job {
            uuid: Uuid::new_v4(),
            source: PathBuf::from("/tmp/test.txt"),
            dest: Some(PathBuf::from("/tmp/test_moved.txt")),
            status: JobStatus::Queued,
            retry_count: 0,
            op_type: OperationType::Move,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        queue.enqueue(&job).unwrap();
        queue.mark_failed(&job.uuid).unwrap();
        queue.mark_failed(&job.uuid).unwrap();

        // Check retry count was incremented
        let mut stmt = queue.conn.prepare(
            "SELECT retry_count FROM jobs WHERE uuid = ?1"
        ).unwrap();
        let retry_count: i32 = stmt.query_row(params![job.uuid.to_string()], |row| row.get(0)).unwrap();
        assert_eq!(retry_count, 2);
    }

    #[test]
    fn test_crash_recovery() {
        let queue = create_test_queue();
        let job1 = Job {
            uuid: Uuid::new_v4(),
            source: PathBuf::from("/tmp/test1.txt"),
            dest: Some(PathBuf::from("/tmp/test1_moved.txt")),
            status: JobStatus::Running,
            retry_count: 0,
            op_type: OperationType::Move,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let job2 = Job {
            uuid: Uuid::new_v4(),
            source: PathBuf::from("/tmp/test2.txt"),
            dest: Some(PathBuf::from("/tmp/test2_moved.txt")),
            status: JobStatus::Running,
            retry_count: 5,
            op_type: OperationType::Move,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        // Manually insert running jobs
        queue.conn.execute(
            "INSERT INTO jobs (uuid, source, dest, status, retry_count, op_type, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            params![
                job1.uuid.to_string(),
                job1.source.to_string_lossy().as_ref(),
                job1.dest.as_ref().map(|p| p.to_string_lossy().to_string()),
                job1.status.to_i32(),
                job1.retry_count,
                job1.op_type.to_i32(),
                job1.created_at.to_rfc3339(),
                job1.updated_at.to_rfc3339(),
            ],
        ).unwrap();

        queue.conn.execute(
            "INSERT INTO jobs (uuid, source, dest, status, retry_count, op_type, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            params![
                job2.uuid.to_string(),
                job2.source.to_string_lossy().as_ref(),
                job2.dest.as_ref().map(|p| p.to_string_lossy().to_string()),
                job2.status.to_i32(),
                job2.retry_count,
                job2.op_type.to_i32(),
                job2.created_at.to_rfc3339(),
                job2.updated_at.to_rfc3339(),
            ],
        ).unwrap();

        // Run crash recovery with max_retries = 3
        queue.crash_recovery(3).unwrap();

        // job1 should be reset to queued (retry_count < max)
        let status1 = queue.get_status(&job1.uuid).unwrap();
        assert_eq!(status1, Some(JobStatus::Queued));

        // job2 should be marked as failed (retry_count >= max)
        let status2 = queue.get_status(&job2.uuid).unwrap();
        assert_eq!(status2, Some(JobStatus::Failed));
    }

    #[test]
    fn test_queue_depth() {
        let queue = create_test_queue();
        
        let job = Job {
            uuid: Uuid::new_v4(),
            source: PathBuf::from("/tmp/test.txt"),
            dest: Some(PathBuf::from("/tmp/test_moved.txt")),
            status: JobStatus::Queued,
            retry_count: 0,
            op_type: OperationType::Move,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        assert_eq!(queue.queue_depth().unwrap(), 0);
        queue.enqueue(&job).unwrap();
        assert_eq!(queue.queue_depth().unwrap(), 1);
    }

    #[test]
    fn test_persistence_across_reopen() {
        let temp_file = NamedTempFile::new().unwrap();
        let db_path = temp_file.path().to_path_buf();

        let job = Job {
            uuid: Uuid::new_v4(),
            source: PathBuf::from("/tmp/test.txt"),
            dest: Some(PathBuf::from("/tmp/test_moved.txt")),
            status: JobStatus::Queued,
            retry_count: 0,
            op_type: OperationType::Move,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        // Enqueue in first connection
        {
            let queue = JobQueue::new(&db_path).unwrap();
            queue.enqueue(&job).unwrap();
        }

        // Reopen and verify job persists
        {
            let queue = JobQueue::new(&db_path).unwrap();
            let status = queue.get_status(&job.uuid).unwrap();
            assert_eq!(status, Some(JobStatus::Queued));
            assert_eq!(queue.queue_depth().unwrap(), 1);
        }
    }
}