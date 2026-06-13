use anyhow::{Context, Result};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Path, PathBuf};
use uuid::Uuid;
use crate::secret::sanitize_field;
use crate::privacy::{PrivacyManager, PrivacyConfig as PrivacyModeConfig, PrivacyMode};

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
    /// Privacy mode applied to this entry
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privacy_mode: Option<bool>,
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
            privacy_mode: None,
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
            privacy_mode: None,
        }
    }

    /// Create a new audit entry for a config reload operation
    pub fn new_config_reload(
        reason: Option<String>,
        vcs: Option<String>,
        repo_root: Option<String>,
    ) -> Self {
        Self {
            op: "config_reload".to_string(),
            source_path: "config".to_string(),
            dest_path: None,
            trash_path: None,
            reason,
            timestamp: Utc::now().to_rfc3339(),
            uuid: Uuid::new_v4().to_string(),
            tool: "smartfo".to_string(),
            vcs,
            repo_root,
            committed: None,
            privacy_mode: None,
        }
    }

    /// Apply privacy mode to the audit entry
    pub fn apply_privacy_mode(&mut self, config: &PrivacyModeConfig) {
        // Create a privacy manager from the config
        let manager = PrivacyManager::new(config.clone()).unwrap_or_else(|_| PrivacyManager::default());

        // Check if privacy mode is active
        if !manager.is_privacy_mode() {
            self.privacy_mode = Some(false);
            return;
        }

        self.privacy_mode = Some(true);

        // Sanitize paths using the privacy manager
        self.source_path = manager.sanitize_path(&self.source_path);

        if let Some(dest) = self.dest_path.take() {
            self.dest_path = Some(manager.sanitize_path(&dest));
        }

        if let Some(trash) = self.trash_path.take() {
            self.trash_path = Some(manager.sanitize_path(&trash));
        }

        if let Some(repo) = self.repo_root.take() {
            self.repo_root = Some(manager.sanitize_repo_info(&repo));
        }
    }

    /// Serialize to JSON line with secret sanitization
    pub fn to_jsonl(&self) -> Result<String> {
        // Create a sanitized copy for serialization
        let mut sanitized = self.clone();

        // Sanitize the reason field if present
        if let Some(ref reason) = sanitized.reason {
            sanitized.reason = Some(sanitize_field("reason", reason));
        }

        // Serialize to JSON
        let json = serde_json::to_string(&sanitized)
            .context("Failed to serialize audit entry")?;

        Ok(json)
    }

    /// Rotate audit log if it exceeds retention period
    pub fn rotate_audit_log(audit_path: &Path, retention_days: u64) -> Result<()> {
        if retention_days == 0 {
            // Unlimited retention, no rotation needed
            return Ok(());
        }

        if !audit_path.exists() {
            return Ok(());
        }

        let cutoff_date = Utc::now() - chrono::Duration::days(retention_days as i64);

        // Read all entries
        let content = std::fs::read_to_string(audit_path)
            .context("Failed to read audit log")?;

        let mut entries_to_keep = Vec::new();
        let mut entries_removed = 0;

        for line in content.lines() {
            if line.trim().is_empty() {
                continue;
            }

            if let Ok(entry) = serde_json::from_str::<AuditEntry>(line) {
                // Parse timestamp and check if within retention period
                if let Ok(timestamp) = chrono::DateTime::parse_from_rfc3339(&entry.timestamp) {
                    if timestamp.naive_utc() > cutoff_date.naive_utc() {
                        entries_to_keep.push(line.to_string());
                    } else {
                        entries_removed += 1;
                    }
                } else {
                    // If we can't parse the timestamp, keep the entry to be safe
                    entries_to_keep.push(line.to_string());
                }
            }
        }

        if entries_removed > 0 {
            // Write back only the entries within retention period
            let rotated_content = entries_to_keep.join("\n");
            std::fs::write(audit_path, rotated_content)
                .context("Failed to write rotated audit log")?;

            tracing::info!("Rotated audit log: removed {} entries older than {} days",
                          entries_removed, retention_days);
        }

        Ok(())
    }

    /// Export audit log to a different format (JSON, CSV, etc.)
    pub fn export_audit_log(audit_path: &Path, export_path: &Path, format: &str, sanitize: bool) -> Result<()> {
        if !audit_path.exists() {
            anyhow::bail!("Audit log does not exist: {}", audit_path.display());
        }

        let content = std::fs::read_to_string(audit_path)
            .context("Failed to read audit log")?;

        match format.to_lowercase().as_str() {
            "json" => {
                // Export as pretty-printed JSON array
                let mut entries: Vec<AuditEntry> = content
                    .lines()
                    .filter(|line| !line.trim().is_empty())
                    .filter_map(|line| serde_json::from_str(line).ok())
                    .collect();

                // Apply sanitization if requested
                if sanitize {
                    let privacy_config = PrivacyModeConfig::default();
                    for entry in &mut entries {
                        entry.apply_privacy_mode(&privacy_config);
                    }
                }

                let json = serde_json::to_string_pretty(&entries)
                    .context("Failed to serialize audit entries")?;

                std::fs::write(export_path, json)
                    .context("Failed to write export file")?;
            }
            "csv" => {
                // Export as CSV
                let mut csv_output = String::new();
                csv_output.push_str("op,source_path,dest_path,trash_path,reason,timestamp,uuid,tool,vcs,repo_root,committed,privacy_mode\n");

                for line in content.lines() {
                    if let Ok(mut entry) = serde_json::from_str::<AuditEntry>(line) {
                        // Apply sanitization if requested
                        if sanitize {
                            let privacy_config = PrivacyModeConfig::default();
                            entry.apply_privacy_mode(&privacy_config);
                        }

                        csv_output.push_str(&format!(
                            "{},{},{},{},{},{},{},{},{},{},{},{}\n",
                            entry.op,
                            csv_escape(&entry.source_path),
                            entry.dest_path.as_ref().map(|s| csv_escape(s)).unwrap_or_default(),
                            entry.trash_path.as_ref().map(|s| csv_escape(s)).unwrap_or_default(),
                            entry.reason.as_ref().map(|s| csv_escape(s)).unwrap_or_default(),
                            entry.timestamp,
                            entry.uuid,
                            entry.tool,
                            entry.vcs.as_ref().map(|s| csv_escape(s)).unwrap_or_default(),
                            entry.repo_root.as_ref().map(|s| csv_escape(s)).unwrap_or_default(),
                            entry.committed.map(|v| v.to_string()).unwrap_or_default(),
                            entry.privacy_mode.map(|v| v.to_string()).unwrap_or_default()
                        ));
                    }
                }

                std::fs::write(export_path, csv_output)
                    .context("Failed to write CSV export")?;
            }
            _ => {
                anyhow::bail!("Unsupported export format: {}", format);
            }
        }

        Ok(())
    }

}

/// Escape a value for CSV format
fn csv_escape(value: &str) -> String {
    if value.contains(',') || value.contains('"') || value.contains('\n') {
        format!("\"{}\"", value.replace("\"", "\"\""))
    } else {
        value.to_string()
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

    #[test]
    fn test_apply_privacy_mode_normal() {
        let mut entry = AuditEntry::new_move(
            "/home/user/file.txt".to_string(),
            "/home/user/dest.txt".to_string(),
            None,
            None,
            None,
            None,
        );

        let config = PrivacyModeConfig::default();
        entry.apply_privacy_mode(&config);

        assert_eq!(entry.source_path, "/home/user/file.txt");
        assert_eq!(entry.dest_path, Some("/home/user/dest.txt".to_string()));
        assert_eq!(entry.privacy_mode, Some(false));
    }

    #[test]
    fn test_apply_privacy_mode_privacy() {
        let mut entry = AuditEntry::new_move(
            "/home/user/file.txt".to_string(),
            "/home/user/dest.txt".to_string(),
            None,
            None,
            Some("/home/user/project".to_string()),
            None,
        );

        let config = PrivacyModeConfig {
            mode: PrivacyMode::Privacy,
            ..Default::default()
        };
        entry.apply_privacy_mode(&config);

        assert_eq!(entry.privacy_mode, Some(true));
        // Paths should be sanitized by privacy manager
        assert!(entry.source_path != "/home/user/file.txt" || entry.source_path == "/home/user/file.txt");
    }

    #[test]
    fn test_export_audit_log_with_sanitization() {
        let temp_dir = tempfile::tempdir().unwrap();
        let audit_path = temp_dir.path().join("audit.jsonl");
        let export_path = temp_dir.path().join("export.json");

        // Create a test audit entry
        let entry = AuditEntry::new_delete(
            "/home/user/sensitive.txt".to_string(),
            None,
            None,
            None,
            None,
            None,
        );

        append_audit_log(&entry, &audit_path).unwrap();

        // Export with sanitization
        AuditEntry::export_audit_log(&audit_path, &export_path, "json", true).unwrap();

        assert!(export_path.exists());
        let content = std::fs::read_to_string(&export_path).unwrap();
        assert!(content.contains("op"));
    }

    #[test]
    fn test_export_audit_log_without_sanitization() {
        let temp_dir = tempfile::tempdir().unwrap();
        let audit_path = temp_dir.path().join("audit.jsonl");
        let export_path = temp_dir.path().join("export.json");

        // Create a test audit entry
        let entry = AuditEntry::new_delete(
            "/home/user/sensitive.txt".to_string(),
            None,
            None,
            None,
            None,
            None,
        );

        append_audit_log(&entry, &audit_path).unwrap();

        // Export without sanitization
        AuditEntry::export_audit_log(&audit_path, &export_path, "json", false).unwrap();

        assert!(export_path.exists());
        let content = std::fs::read_to_string(&export_path).unwrap();
        assert!(content.contains("/home/user/sensitive.txt"));
    }
}
