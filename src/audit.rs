use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::Path;
use uuid::Uuid;

/// Audit entry for every mv/rm operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEntry {
    /// Operation type: "move" or "delete"
    pub op: String,
    /// Source path (original location)
    pub source_path: String,
    /// Destination path (for moves) or trash path (for deletes)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dest_path: Option<String>,
    /// Trash path (for deletes only)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trash_path: Option<String>,
    /// User-provided reason for the operation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// ISO 8601 timestamp
    pub timestamp: String,
    /// Unique operation identifier
    pub uuid: String,
    /// Tool name (always "smartfo")
    pub tool: String,
    /// VCS system if applicable (git, hg, svn, jj)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vcs: Option<String>,
    /// VCS repository root if applicable
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repo_root: Option<String>,
    /// Whether the file was VCS-committed (no uncommitted changes)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub committed: Option<bool>,
}

impl AuditEntry {
    /// Create a new audit entry for a move operation
    pub fn new_move(
        source_path: String,
        dest_path: String,
        reason: Option<String>,
        vcs: Option<String>,
        repo_root: Option<String>,
        committed: Option<bool>,
    ) -> Self {
        Self {
            op: "move".to_string(),
            source_path,
            dest_path: Some(dest_path),
            trash_path: None,
            reason,
            timestamp: Utc::now().to_rfc3339(),
            uuid: Uuid::new_v4().to_string(),
            tool: "smartfo".to_string(),
            vcs,
            repo_root,
            committed,
        }
    }

    /// Create a new audit entry for a delete operation
    pub fn new_delete(
        source_path: String,
        trash_path: Option<String>,
        reason: Option<String>,
        vcs: Option<String>,
        repo_root: Option<String>,
        committed: Option<bool>,
    ) -> Self {
        Self {
            op: "delete".to_string(),
            source_path,
            dest_path: None,
            trash_path,
            reason,
            timestamp: Utc::now().to_rfc3339(),
            uuid: Uuid::new_v4().to_string(),
            tool: "smartfo".to_string(),
            vcs,
            repo_root,
            committed,
        }
    }

    /// Serialize to JSON line
    pub fn to_jsonl(&self) -> Result<String> {
        serde_json::to_string(self)
            .context("Failed to serialize audit entry to JSON")
    }
}

/// Append an audit entry to the audit log file
///
/// Creates parent directories if they don't exist.
/// Uses atomic append to handle concurrent writes safely.
pub fn append_audit_log(entry: &AuditEntry, audit_log_path: &Path) -> Result<()> {
    // Ensure parent directory exists
    if let Some(parent) = audit_log_path.parent() {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("Failed to create audit log directory: {}", parent.display()))?;
    }

    // Open file in append mode, create if it doesn't exist
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .write(true)
        .open(audit_log_path)
        .with_context(|| format!("Failed to open audit log: {}", audit_log_path.display()))?;

    // Serialize and write the entry
    let json_line = entry.to_jsonl()?;
    writeln!(file, "{}", json_line)
        .with_context(|| format!("Failed to write to audit log: {}", audit_log_path.display()))?;

    // Ensure data is flushed to disk
    file.flush()
        .context("Failed to flush audit log to disk")?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[test]
    fn test_audit_entry_move_serialization() {
        let entry = AuditEntry::new_move(
            "/tmp/source.txt".to_string(),
            "/tmp/dest.txt".to_string(),
            Some("refactor: moved to new location".to_string()),
            Some("git".to_string()),
            Some("/home/user/project".to_string()),
            Some(true),
        );

        let json = entry.to_jsonl().unwrap();
        assert!(json.contains("\"op\":\"move\""));
        assert!(json.contains("\"source_path\":\"/tmp/source.txt\""));
        assert!(json.contains("\"dest_path\":\"/tmp/dest.txt\""));
        assert!(json.contains("\"reason\":\"refactor: moved to new location\""));
        assert!(json.contains("\"vcs\":\"git\""));
    }

    #[test]
    fn test_audit_entry_delete_serialization() {
        let entry = AuditEntry::new_delete(
            "/tmp/old.txt".to_string(),
            Some("/trash/old.txt".to_string()),
            None,
            None,
            None,
            None,
        );

        let json = entry.to_jsonl().unwrap();
        assert!(json.contains("\"op\":\"delete\""));
        assert!(json.contains("\"source_path\":\"/tmp/old.txt\""));
        assert!(json.contains("\"trash_path\":\"/trash/old.txt\""));
        assert!(!json.contains("\"vcs\"")); // None fields should be skipped
    }

    #[test]
    fn test_append_audit_log_creates_directory() {
        let temp_dir = tempfile::tempdir().unwrap();
        let audit_path = temp_dir.path().join("nested/audit/operations.jsonl");

        let entry = AuditEntry::new_delete(
            "/tmp/test.txt".to_string(),
            None,
            None,
            None,
            None,
            None,
        );

        append_audit_log(&entry, &audit_path).unwrap();
        assert!(audit_path.exists());
    }

    #[test]
    fn test_append_audit_log_multiple_entries() {
        let temp_file = NamedTempFile::new().unwrap();
        let audit_path = temp_file.path();

        let entry1 = AuditEntry::new_delete("/tmp/file1.txt".to_string(), None, None, None, None, None);
        let entry2 = AuditEntry::new_delete("/tmp/file2.txt".to_string(), None, None, None, None, None);

        append_audit_log(&entry1, audit_path).unwrap();
        append_audit_log(&entry2, audit_path).unwrap();

        let content = std::fs::read_to_string(audit_path).unwrap();
        let lines: Vec<&str> = content.lines().collect();
        assert_eq!(lines.len(), 2);
    }

    #[test]
    fn test_audit_entry_uuid_unique() {
        let entry1 = AuditEntry::new_delete("/tmp/file1.txt".to_string(), None, None, None, None, None);
        let entry2 = AuditEntry::new_delete("/tmp/file2.txt".to_string(), None, None, None, None, None);

        assert_ne!(entry1.uuid, entry2.uuid);
    }
}
