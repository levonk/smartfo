use std::path::{Path, PathBuf};
use chrono::{DateTime, Utc};
use crate::config::Config;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Entry in the .smartfo-index JSONL file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrashIndexEntry {
    /// Original absolute path before deletion
    pub original_path: PathBuf,
    /// Timestamp of deletion (ISO 8601)
    pub timestamp: DateTime<Utc>,
    /// Unique identifier for this deletion
    pub uuid: String,
    /// Reason for deletion (e.g., "rm", "mv", "cleanup")
    pub reason: String,
}

impl TrashIndexEntry {
    /// Create a new trash index entry
    pub fn new(original_path: PathBuf, reason: String) -> Self {
        Self {
            original_path,
            timestamp: Utc::now(),
            uuid: Uuid::new_v4().to_string(),
            reason,
        }
    }

    /// Convert to JSONL string
    pub fn to_jsonl(&self) -> Result<String> {
        serde_json::to_string(self)
            .context("Failed to serialize trash index entry to JSON")
    }
}

/// Trash directory manager that computes versioned trash paths
pub struct TrashManager {
    trash_root: PathBuf,
}

impl TrashManager {
    /// Create a new TrashManager with the given configuration
    pub fn new(config: &Config) -> Self {
        let trash_root = config.trash.root.clone();
        TrashManager { trash_root }
    }

    /// Compute the versioned trash path for a given source file
    /// 
    /// Format: `$TRASH_ROOT/<abs-path>/<iso-timestamp>-<counter>/<filename>`
    /// 
    /// # Arguments
    /// * `source_path` - The absolute path to the file being trashed
    /// * `counter` - A counter to handle rapid successive deletes (default: 001)
    /// 
    /// # Returns
    /// The full path where the file should be moved in trash
    pub fn compute_trash_path(&self, source_path: &Path, counter: u32) -> PathBuf {
        // Get absolute path and strip leading '/' for the trash subdirectory structure
        let abs_path = source_path.canonicalize().unwrap_or_else(|_| source_path.to_path_buf());
        let path_str = abs_path.to_string_lossy();
        let relative_path = path_str.strip_prefix('/').unwrap_or(&path_str);

        // Generate timestamp in ISO 8601 format
        let timestamp: DateTime<Utc> = Utc::now();
        let timestamp_str = timestamp.format("%Y-%m-%dT%H:%M:%SZ").to_string();

        // Format counter as three-digit zero-padded number
        let counter_str = format!("{:03}", counter);

        // Build the trash path: $TRASH_ROOT/<abs-path>/<timestamp>-<counter>/<filename>
        let filename = source_path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown");

        let trash_path = self.trash_root
            .join(relative_path)
            .join(format!("{}-{}", timestamp_str, counter_str))
            .join(filename);

        trash_path
    }

    /// Create the parent directory structure for a trash path
    /// 
    /// This ensures all necessary parent directories exist before moving a file to trash.
    /// 
    /// # Arguments
    /// * `trash_path` - The full trash path where a file will be moved
    /// 
    /// # Returns
    /// Ok(()) if directories were created successfully, Err otherwise
    pub fn create_parent_dirs(&self, trash_path: &Path) -> Result<()> {
        if let Some(parent) = trash_path.parent() {
            std::fs::create_dir_all(parent)
                .with_context(|| format!("Failed to create trash parent directories: {}", parent.display()))?;
        }
        Ok(())
    }

    /// Move a file to trash with atomic operations
    /// 
    /// - If source and trash are on the same filesystem: uses atomic rename
    /// - If cross-device: uses copy + fsync + unlink (to be implemented in later story)
    /// 
    /// # Arguments
    /// * `source_path` - The absolute path to the file to move
    /// * `trash_path` - The destination path in trash
    /// * `reason` - The reason for deletion (for index entry)
    /// 
    /// # Returns
    /// Ok(()) if the move was successful, Err otherwise
    pub fn move_to_trash(&self, source_path: &Path, trash_path: &Path, reason: &str) -> Result<()> {
        // Call before-trash hook
        self.before_trash_hook(source_path)?;

        // Create parent directories first
        self.create_parent_dirs(trash_path)?;

        // Try atomic rename first (same filesystem)
        std::fs::rename(source_path, trash_path)
            .with_context(|| format!(
                "Failed to move {} to trash at {}",
                source_path.display(),
                trash_path.display()
            ))?;

        // Append index entry
        self.append_index_entry(source_path, reason)?;

        // Call after-trash hook
        self.after_trash_hook(source_path, trash_path)?;

        Ok(())
    }

    /// Get the trash root directory
    pub fn trash_root(&self) -> &Path {
        &self.trash_root
    }

    /// Get the parent directory path for a given source file in trash
    /// (without the timestamp-counter suffix)
    pub fn get_trash_parent(&self, source_path: &Path) -> PathBuf {
        let abs_path = source_path.canonicalize().unwrap_or_else(|_| source_path.to_path_buf());
        let path_str = abs_path.to_string_lossy();
        let relative_path = path_str.strip_prefix('/').unwrap_or(&path_str);
        self.trash_root.join(relative_path)
    }

    /// Get the path to the .smartfo-index file for a given source path
    pub fn get_index_path(&self, source_path: &Path) -> PathBuf {
        let trash_parent = self.get_trash_parent(source_path);
        trash_parent.join(".smartfo-index")
    }

    /// Append an entry to the .smartfo-index file
    /// 
    /// # Arguments
    /// * `source_path` - The original path of the deleted file
    /// * `reason` - The reason for deletion
    /// 
    /// # Returns
    /// Ok(()) if the entry was appended successfully, Err otherwise
    pub fn append_index_entry(&self, source_path: &Path, reason: &str) -> Result<()> {
        let index_path = self.get_index_path(source_path);
        
        // Ensure parent directory exists
        if let Some(parent) = index_path.parent() {
            std::fs::create_dir_all(parent)
                .with_context(|| format!("Failed to create index parent directory: {}", parent.display()))?;
        }

        // Create the index entry
        let abs_path = source_path.canonicalize().unwrap_or_else(|_| source_path.to_path_buf());
        let entry = TrashIndexEntry::new(abs_path, reason.to_string());
        
        // Append to the index file
        let jsonl = entry.to_jsonl()?;
        let mut file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&index_path)
            .with_context(|| format!("Failed to open index file: {}", index_path.display()))?;
        
        use std::io::Write;
        writeln!(file, "{}", jsonl)
            .with_context(|| format!("Failed to write to index file: {}", index_path.display()))?;

        Ok(())
    }

    /// Hook called before moving a file to trash
    /// 
    /// This hook can be used to check retention policies, disk space, etc.
    /// Full implementation in story 05-002 (disk space guard and auto-culling).
    /// 
    /// # Arguments
    /// * `source_path` - The path of the file to be trashed
    /// 
    /// # Returns
    /// Ok(()) if the file can be trashed, Err otherwise
    pub fn before_trash_hook(&self, _source_path: &Path) -> Result<()> {
        // TODO: Implement retention policy check in story 05-002
        // TODO: Implement disk space check in story 05-002
        Ok(())
    }

    /// Hook called after moving a file to trash
    /// 
    /// This hook can be used to trigger cleanup if needed.
    /// Full implementation in story 05-002 (disk space guard and auto-culling).
    /// 
    /// # Arguments
    /// * `source_path` - The original path of the trashed file
    /// * `trash_path` - The path where the file was moved in trash
    /// 
    /// # Returns
    /// Ok(()) if post-trash operations succeeded, Err otherwise
    pub fn after_trash_hook(&self, _source_path: &Path, _trash_path: &Path) -> Result<()> {
        // TODO: Implement auto-cleanup trigger in story 05-002
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_trash_path_computation() {
        let temp_dir = TempDir::new().unwrap();
        let trash_root = temp_dir.path().join("trash");
        fs::create_dir_all(&trash_root).unwrap();

        let mut config = Config::default();
        config.trash.root = trash_root.clone();

        let manager = TrashManager::new(&config);
        let source_path = PathBuf::from("/home/user/foo.txt");

        let trash_path = manager.compute_trash_path(&source_path, 1);

        // Verify the path structure
        assert!(trash_path.starts_with(&trash_root));
        assert!(trash_path.to_string_lossy().contains("home/user/foo.txt"));
        assert!(trash_path.to_string_lossy().contains("-001"));
        assert!(trash_path.ends_with("foo.txt"));
    }

    #[test]
    fn test_trash_path_with_counter() {
        let temp_dir = TempDir::new().unwrap();
        let trash_root = temp_dir.path().join("trash");
        fs::create_dir_all(&trash_root).unwrap();

        let mut config = Config::default();
        config.trash.root = trash_root.clone();

        let manager = TrashManager::new(&config);
        let source_path = PathBuf::from("/home/user/bar.txt");

        let path1 = manager.compute_trash_path(&source_path, 1);
        let path2 = manager.compute_trash_path(&source_path, 2);

        // Verify different counters produce different paths
        assert_ne!(path1, path2);
        assert!(path1.to_string_lossy().contains("-001"));
        assert!(path2.to_string_lossy().contains("-002"));
    }

    #[test]
    fn test_trash_root() {
        let temp_dir = TempDir::new().unwrap();
        let trash_root = temp_dir.path().join("trash");
        fs::create_dir_all(&trash_root).unwrap();

        let mut config = Config::default();
        config.trash.root = trash_root.clone();

        let manager = TrashManager::new(&config);
        assert_eq!(manager.trash_root(), &trash_root);
    }

    #[test]
    fn test_get_trash_parent() {
        let temp_dir = TempDir::new().unwrap();
        let trash_root = temp_dir.path().join("trash");
        fs::create_dir_all(&trash_root).unwrap();

        let mut config = Config::default();
        config.trash.root = trash_root.clone();

        let manager = TrashManager::new(&config);
        let source_path = PathBuf::from("/home/user/test.txt");

        let parent = manager.get_trash_parent(&source_path);
        let expected = trash_root.join("home/user/test.txt");
        assert_eq!(parent, expected);
    }

    #[test]
    fn test_create_parent_dirs() {
        let temp_dir = TempDir::new().unwrap();
        let trash_root = temp_dir.path().join("trash");
        fs::create_dir_all(&trash_root).unwrap();

        let mut config = Config::default();
        config.trash.root = trash_root.clone();

        let manager = TrashManager::new(&config);
        let source_path = PathBuf::from("/home/user/test.txt");
        let trash_path = manager.compute_trash_path(&source_path, 1);

        // Create parent directories
        manager.create_parent_dirs(&trash_path).unwrap();

        // Verify the parent directory exists
        let parent = trash_path.parent().unwrap();
        assert!(parent.exists());
        assert!(parent.is_dir());
    }

    #[test]
    fn test_create_parent_dirs_nested() {
        let temp_dir = TempDir::new().unwrap();
        let trash_root = temp_dir.path().join("trash");
        fs::create_dir_all(&trash_root).unwrap();

        let mut config = Config::default();
        config.trash.root = trash_root.clone();

        let manager = TrashManager::new(&config);
        let source_path = PathBuf::from("/deep/nested/path/to/file.txt");
        let trash_path = manager.compute_trash_path(&source_path, 1);

        // Create parent directories for nested path
        manager.create_parent_dirs(&trash_path).unwrap();

        // Verify the parent directory exists
        let parent = trash_path.parent().unwrap();
        assert!(parent.exists());
        assert!(parent.is_dir());
    }

    #[test]
    fn test_move_to_trash() {
        let temp_dir = TempDir::new().unwrap();
        let trash_root = temp_dir.path().join("trash");
        fs::create_dir_all(&trash_root).unwrap();

        // Create a test file to move
        let source_file = temp_dir.path().join("test_file.txt");
        fs::write(&source_file, "test content").unwrap();

        let mut config = Config::default();
        config.trash.root = trash_root.clone();

        let manager = TrashManager::new(&config);
        let trash_path = manager.compute_trash_path(&source_file, 1);

        // Move file to trash
        manager.move_to_trash(&source_file, &trash_path, "rm").unwrap();

        // Verify source file no longer exists
        assert!(!source_file.exists());

        // Verify file exists in trash
        assert!(trash_path.exists());

        // Verify content is preserved
        let content = fs::read_to_string(&trash_path).unwrap();
        assert_eq!(content, "test content");
    }

    #[test]
    fn test_move_to_trash_creates_dirs() {
        let temp_dir = TempDir::new().unwrap();
        let trash_root = temp_dir.path().join("trash");
        fs::create_dir_all(&trash_root).unwrap();

        // Create a test file to move
        let source_file = temp_dir.path().join("test_file.txt");
        fs::write(&source_file, "test content").unwrap();

        let mut config = Config::default();
        config.trash.root = trash_root.clone();

        let manager = TrashManager::new(&config);
        let trash_path = manager.compute_trash_path(&source_file, 1);

        // Move file to trash (should create parent dirs automatically)
        manager.move_to_trash(&source_file, &trash_path, "rm").unwrap();

        // Verify file exists in trash
        assert!(trash_path.exists());
    }

    #[test]
    fn test_before_trash_hook() {
        let temp_dir = TempDir::new().unwrap();
        let trash_root = temp_dir.path().join("trash");
        fs::create_dir_all(&trash_root).unwrap();

        let source_file = temp_dir.path().join("test_file.txt");
        fs::write(&source_file, "test content").unwrap();

        let mut config = Config::default();
        config.trash.root = trash_root.clone();

        let manager = TrashManager::new(&config);
        
        // Hook should succeed (stub implementation)
        assert!(manager.before_trash_hook(&source_file).is_ok());
    }

    #[test]
    fn test_after_trash_hook() {
        let temp_dir = TempDir::new().unwrap();
        let trash_root = temp_dir.path().join("trash");
        fs::create_dir_all(&trash_root).unwrap();

        let source_file = temp_dir.path().join("test_file.txt");
        let trash_path = temp_dir.path().join("trash/test_file.txt");

        let mut config = Config::default();
        config.trash.root = trash_root.clone();

        let manager = TrashManager::new(&config);
        
        // Hook should succeed (stub implementation)
        assert!(manager.after_trash_hook(&source_file, &trash_path).is_ok());
    }

    #[test]
    fn test_index_entry_creation() {
        let original_path = PathBuf::from("/home/user/test.txt");
        let reason = "rm".to_string();
        
        let entry = TrashIndexEntry::new(original_path.clone(), reason.clone());
        
        assert_eq!(entry.original_path, original_path);
        assert_eq!(entry.reason, reason);
        assert!(!entry.uuid.is_empty());
        
        // Verify timestamp is recent (within last minute)
        let now = Utc::now();
        let duration = now.signed_duration_since(entry.timestamp);
        assert!(duration.num_seconds() < 60);
    }

    #[test]
    fn test_index_entry_to_jsonl() {
        let original_path = PathBuf::from("/home/user/test.txt");
        let entry = TrashIndexEntry::new(original_path, "rm".to_string());
        
        let jsonl = entry.to_jsonl().unwrap();
        
        // Verify it's valid JSON
        let parsed: serde_json::Value = serde_json::from_str(&jsonl).unwrap();
        assert!(parsed.is_object());
        assert_eq!(parsed["original_path"], "/home/user/test.txt");
        assert_eq!(parsed["reason"], "rm");
        assert!(parsed["uuid"].is_string());
        assert!(parsed["timestamp"].is_string());
    }

    #[test]
    fn test_append_index_entry() {
        let temp_dir = TempDir::new().unwrap();
        let trash_root = temp_dir.path().join("trash");
        fs::create_dir_all(&trash_root).unwrap();

        let source_file = temp_dir.path().join("test_file.txt");
        fs::write(&source_file, "test content").unwrap();

        let mut config = Config::default();
        config.trash.root = trash_root.clone();

        let manager = TrashManager::new(&config);
        
        // Append index entry
        manager.append_index_entry(&source_file, "rm").unwrap();

        // Verify index file exists
        let index_path = manager.get_index_path(&source_file);
        assert!(index_path.exists());

        // Verify index file contains valid JSONL
        let content = fs::read_to_string(&index_path).unwrap();
        let lines: Vec<&str> = content.lines().collect();
        assert_eq!(lines.len(), 1);
        
        let parsed: serde_json::Value = serde_json::from_str(lines[0]).unwrap();
        assert_eq!(parsed["reason"], "rm");
    }

    #[test]
    fn test_append_multiple_index_entries() {
        let temp_dir = TempDir::new().unwrap();
        let trash_root = temp_dir.path().join("trash");
        fs::create_dir_all(&trash_root).unwrap();

        let source_file = temp_dir.path().join("test_file.txt");
        fs::write(&source_file, "test content").unwrap();

        let mut config = Config::default();
        config.trash.root = trash_root.clone();

        let manager = TrashManager::new(&config);
        
        // Append multiple entries
        manager.append_index_entry(&source_file, "rm").unwrap();
        manager.append_index_entry(&source_file, "mv").unwrap();

        // Verify index file contains both entries
        let index_path = manager.get_index_path(&source_file);
        let content = fs::read_to_string(&index_path).unwrap();
        let lines: Vec<&str> = content.lines().collect();
        assert_eq!(lines.len(), 2);
        
        // Verify each line is valid JSON
        for line in lines {
            let parsed: serde_json::Value = serde_json::from_str(line).unwrap();
            assert!(parsed.is_object());
        }
    }

    #[test]
    fn test_same_file_history_preservation() {
        let temp_dir = TempDir::new().unwrap();
        let trash_root = temp_dir.path().join("trash");
        fs::create_dir_all(&trash_root).unwrap();

        let source_file = temp_dir.path().join("test_file.txt");
        
        let mut config = Config::default();
        config.trash.root = trash_root.clone();

        let manager = TrashManager::new(&config);
        
        // Compute trash paths for the same file with different counters
        let path1 = manager.compute_trash_path(&source_file, 1);
        let path2 = manager.compute_trash_path(&source_file, 2);
        let path3 = manager.compute_trash_path(&source_file, 3);

        // Verify each path is different (different timestamped subdirs)
        assert_ne!(path1, path2);
        assert_ne!(path2, path3);
        assert_ne!(path1, path3);

        // Verify all paths end with the same filename
        assert_eq!(path1.file_name(), path2.file_name());
        assert_eq!(path2.file_name(), path3.file_name());

        // Verify all paths have different parent directories (timestamped)
        let parent1 = path1.parent().unwrap();
        let parent2 = path2.parent().unwrap();
        let parent3 = path3.parent().unwrap();
        
        assert_ne!(parent1, parent2);
        assert_ne!(parent2, parent3);
        assert_ne!(parent1, parent3);
    }
}