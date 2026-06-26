use anyhow::{Context, Result};
use std::fs;
use std::path::{Path, PathBuf};
use tracing::{debug, info};

/// Represents a change detected in Git staged changes
#[derive(Debug, Clone)]
pub enum GitChange {
    /// File was deleted
    Deletion { path: String },
    /// File was added
    Addition { path: String },
    /// File was modified
    Modification { path: String },
    /// File was renamed (old_path -> new_path)
    Rename { old_path: String, new_path: String },
}

/// Detect Git changes from staged files
pub fn detect_staged_changes(repo_root: &Path) -> Result<Vec<GitChange>> {
    debug!("Detecting staged changes in repo: {}", repo_root.display());

    let mut changes = Vec::new();

    // Get list of staged files using git diff --cached --name-status
    let output = std::process::Command::new("git")
        .args(["diff", "--cached", "--name-status"])
        .current_dir(repo_root)
        .output()
        .context("Failed to execute git diff --cached")?;

    if !output.status.success() {
        anyhow::bail!("git diff --cached failed: {}", String::from_utf8_lossy(&output.stderr));
    }

    let stdout = String::from_utf8(output.stdout)
        .context("Failed to parse git diff output as UTF-8")?;

    // Parse the output: each line is "STATUS\tPATH" or "STATUS\tOLD_PATH\tNEW_PATH"
    for line in stdout.lines() {
        let parts: Vec<&str> = line.split('\t').collect();
        if parts.is_empty() {
            continue;
        }

        let status = parts[0];
        match status {
            "D" => {
                // Deleted file
                if parts.len() >= 2 {
                    changes.push(GitChange::Deletion {
                        path: parts[1].to_string(),
                    });
                }
            }
            "A" => {
                // Added file
                if parts.len() >= 2 {
                    changes.push(GitChange::Addition {
                        path: parts[1].to_string(),
                    });
                }
            }
            "M" => {
                // Modified file
                if parts.len() >= 2 {
                    changes.push(GitChange::Modification {
                        path: parts[1].to_string(),
                    });
                }
            }
            "R" => {
                // Renamed file (git uses R100 for 100% similarity)
                if parts.len() >= 3 {
                    changes.push(GitChange::Rename {
                        old_path: parts[1].to_string(),
                        new_path: parts[2].to_string(),
                    });
                }
            }
            _ => {
                // Other statuses (T, C, etc.) - ignore for now
                debug!("Ignoring git status: {}", status);
            }
        }
    }

    debug!("Detected {} staged changes", changes.len());
    Ok(changes)
}

/// Get the path to the repo-local audit log
pub fn repo_audit_log_path(repo_root: &Path) -> PathBuf {
    repo_root.join(".smartfo/audit/operations.jsonl")
}

/// Read audit entries from the repo-local audit log
pub fn read_repo_audit_log(repo_root: &Path) -> Result<Vec<crate::audit::AuditEntry>> {
    let audit_path = repo_audit_log_path(repo_root);

    if !audit_path.exists() {
        debug!("Audit log does not exist: {}", audit_path.display());
        return Ok(Vec::new());
    }

    let content = fs::read_to_string(&audit_path)
        .with_context(|| format!("Failed to read audit log: {}", audit_path.display()))?;

    let mut entries = Vec::new();
    for line in content.lines() {
        if line.trim().is_empty() {
            continue;
        }
        let entry: crate::audit::AuditEntry = serde_json::from_str(line)
            .with_context(|| format!("Failed to parse audit entry: {}", line))?;
        entries.push(entry);
    }

    debug!("Read {} audit entries from {}", entries.len(), audit_path.display());
    Ok(entries)
}

/// Check if a file deletion has a corresponding audit entry
pub fn has_audit_entry_for_deletion(
    audit_entries: &[crate::audit::AuditEntry],
    file_path: &str,
) -> bool {
    audit_entries.iter().any(|entry| {
        entry.op == "delete" && entry.source_path == file_path
    })
}

/// Check if a file rename has a corresponding audit entry
pub fn has_audit_entry_for_rename(
    audit_entries: &[crate::audit::AuditEntry],
    old_path: &str,
    new_path: &str,
) -> bool {
    audit_entries.iter().any(|entry| {
        entry.op == "move" && entry.source_path == old_path && entry.dest_path.as_deref() == Some(new_path)
    })
}

/// Run the client-side pre-commit hook
pub fn run_pre_commit_hook(repo_root: &Path) -> Result<()> {
    info!("Running pre-commit hook for repo: {}", repo_root.display());

    // Detect staged changes
    let changes = detect_staged_changes(repo_root)?;

    // Read audit log
    let audit_entries = read_repo_audit_log(repo_root)?;

    // Check for raw deletions
    for change in &changes {
        match change {
            GitChange::Deletion { path } => {
                if !has_audit_entry_for_deletion(&audit_entries, path) {
                    anyhow::bail!(
                        "Raw deletion detected: {}\n\
                         This file was deleted without using smartfo.\n\
                         Please use 'smartfo rm' or 'srm' to delete files in a VCS-aware manner.\n\
                         If you intentionally want to delete this file, use 'git commit --no-verify' to bypass this hook.",
                        path
                    );
                }
            }
            GitChange::Rename { old_path, new_path } => {
                if !has_audit_entry_for_rename(&audit_entries, old_path, new_path) {
                    anyhow::bail!(
                        "Raw rename detected: {} -> {}\n\
                         This file was renamed without using smartfo.\n\
                         Please use 'smartfo mv' or 'smv' to move files in a VCS-aware manner.\n\
                         If you intentionally want to rename this file, use 'git commit --no-verify' to bypass this hook.",
                        old_path, new_path
                    );
                }
            }
            _ => {
                // Additions and modifications are fine
            }
        }
    }

    // Check SKILL.md staleness if it exists
    let skill_path = repo_root.join("SKILL.md");
    if skill_path.exists() {
        let skill_content = fs::read_to_string(&skill_path)
            .with_context(|| format!("Failed to read SKILL.md: {}", skill_path.display()))?;

        let is_stale = crate::skill::check_skill_stale(&skill_content)
            .context("Failed to check skill staleness")?;

        if is_stale {
            anyhow::bail!(
                "SKILL.md is stale and needs regeneration.\n\
                 Run 'smartfo agent generate-skill --output SKILL.md' to regenerate it.\n\
                 If you intentionally want to commit the stale skill, use 'git commit --no-verify' to bypass this hook."
            );
        }
    }

    // Automatically update Cargo.lock if Cargo.toml is modified
    let cargo_toml_modified = changes.iter().any(|change| {
        matches!(change, GitChange::Modification { path } | GitChange::Addition { path } if *path == "Cargo.toml")
    });

    if cargo_toml_modified {
        debug!("Cargo.toml modified, running just sync-deps to sync Cargo.lock");
        
        let sync_output = std::process::Command::new("just")
            .args(["sync-deps"])
            .current_dir(repo_root)
            .output()
            .context("Failed to run just sync-deps. Is just installed?")?;

        if !sync_output.status.success() {
            anyhow::bail!(
                "just sync-deps failed: {}\n\
                 Please fix dependency issues manually or use 'git commit --no-verify' to bypass.",
                String::from_utf8_lossy(&sync_output.stderr)
            );
        }

        debug!("Dependencies synchronized via justfile");
    }

    info!("Pre-commit hook check passed");
    Ok(())
}

/// Represents a change in a Git push
#[derive(Debug, Clone)]
pub enum PushChange {
    /// File was deleted
    Deletion { path: String },
    /// File was added
    Addition { path: String },
    /// File was modified
    Modification { path: String },
    /// File was renamed (old_path -> new_path)
    Rename { old_path: String, new_path: String },
}

/// Detect changes from a Git push by comparing old and new refs
pub fn detect_push_changes(repo_root: &Path, old_ref: &str, new_ref: &str) -> Result<Vec<PushChange>> {
    debug!("Detecting push changes: {} -> {}", old_ref, new_ref);

    let mut changes = Vec::new();

    // Use git diff --name-status to detect changes between refs
    let output = std::process::Command::new("git")
        .args(["diff", "--name-status", old_ref, new_ref])
        .current_dir(repo_root)
        .output()
        .context("Failed to execute git diff")?;

    if !output.status.success() {
        anyhow::bail!("git diff failed: {}", String::from_utf8_lossy(&output.stderr));
    }

    let stdout = String::from_utf8(output.stdout)
        .context("Failed to parse git diff output as UTF-8")?;

    // Parse the output: each line is "STATUS\tPATH" or "STATUS\tOLD_PATH\tNEW_PATH"
    for line in stdout.lines() {
        let parts: Vec<&str> = line.split('\t').collect();
        if parts.is_empty() {
            continue;
        }

        let status = parts[0];
        match status {
            "D" => {
                // Deleted file
                if parts.len() >= 2 {
                    changes.push(PushChange::Deletion {
                        path: parts[1].to_string(),
                    });
                }
            }
            "A" => {
                // Added file
                if parts.len() >= 2 {
                    changes.push(PushChange::Addition {
                        path: parts[1].to_string(),
                    });
                }
            }
            "M" => {
                // Modified file
                if parts.len() >= 2 {
                    changes.push(PushChange::Modification {
                        path: parts[1].to_string(),
                    });
                }
            }
            "R" => {
                // Renamed file (git uses R100 for 100% similarity)
                if parts.len() >= 3 {
                    changes.push(PushChange::Rename {
                        old_path: parts[1].to_string(),
                        new_path: parts[2].to_string(),
                    });
                }
            }
            _ => {
                // Other statuses (T, C, etc.) - ignore for now
                debug!("Ignoring git status: {}", status);
            }
        }
    }

    debug!("Detected {} push changes", changes.len());
    Ok(changes)
}

/// Run the server-side pre-receive hook
pub fn run_pre_receive_hook(repo_root: &Path) -> Result<()> {
    info!("Running pre-receive hook for repo: {}", repo_root.display());

    // Read from stdin: each line is "old_sha new_sha ref_name"
    let stdin = std::io::stdin();
    let mut input = String::new();
    stdin.read_line(&mut input)
        .context("Failed to read from stdin")?;

    let parts: Vec<&str> = input.trim().split_whitespace().collect();
    if parts.len() < 3 {
        anyhow::bail!("Invalid pre-receive input format");
    }

    let old_ref = parts[0];
    let new_ref = parts[1];
    let _ref_name = parts[2];

    debug!("Pre-receive: {} -> {} ({})", old_ref, new_ref, _ref_name);

    // Detect changes in the push
    let changes = detect_push_changes(repo_root, old_ref, new_ref)?;

    // Read audit log
    let audit_entries = read_repo_audit_log(repo_root)?;

    // Check for raw deletions and renames
    for change in &changes {
        match change {
            PushChange::Deletion { path } => {
                if !has_audit_entry_for_deletion(&audit_entries, path) {
                    anyhow::bail!(
                        "Push rejected: raw deletion detected in {}\n\
                         This file was deleted without using smartfo.\n\
                         Please use 'smartfo rm' or 'srm' to delete files in a VCS-aware manner before pushing.",
                        path
                    );
                }
            }
            PushChange::Rename { old_path, new_path } => {
                if !has_audit_entry_for_rename(&audit_entries, old_path, new_path) {
                    anyhow::bail!(
                        "Push rejected: raw rename detected: {} -> {}\n\
                         This file was renamed without using smartfo.\n\
                         Please use 'smartfo mv' or 'smv' to move files in a VCS-aware manner before pushing.",
                        old_path, new_path
                    );
                }
            }
            _ => {
                // Additions and modifications are fine
            }
        }
    }

    info!("Pre-receive hook check passed");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use std::fs;

    #[test]
    fn test_repo_audit_log_path() {
        let temp_dir = TempDir::new().unwrap();
        let repo_root = temp_dir.path();
        let audit_path = repo_audit_log_path(repo_root);

        assert_eq!(
            audit_path,
            repo_root.join(".smartfo/audit/operations.jsonl")
        );
    }

    #[test]
    fn test_has_audit_entry_for_deletion() {
        let entries = vec![
            crate::audit::AuditEntry::new_delete("/tmp/file1.txt".to_string(), None, None, None, None, None),
            crate::audit::AuditEntry::new_delete("/tmp/file2.txt".to_string(), None, None, None, None, None),
        ];

        assert!(has_audit_entry_for_deletion(&entries, "/tmp/file1.txt"));
        assert!(has_audit_entry_for_deletion(&entries, "/tmp/file2.txt"));
        assert!(!has_audit_entry_for_deletion(&entries, "/tmp/file3.txt"));
    }

    #[test]
    fn test_has_audit_entry_for_rename() {
        let entries = vec![
            crate::audit::AuditEntry::new_move("/tmp/old1.txt".to_string(), "/tmp/new1.txt".to_string(), None, None, None, None),
            crate::audit::AuditEntry::new_move("/tmp/old2.txt".to_string(), "/tmp/new2.txt".to_string(), None, None, None, None),
        ];

        assert!(has_audit_entry_for_rename(&entries, "/tmp/old1.txt", "/tmp/new1.txt"));
        assert!(has_audit_entry_for_rename(&entries, "/tmp/old2.txt", "/tmp/new2.txt"));
        assert!(!has_audit_entry_for_rename(&entries, "/tmp/old1.txt", "/tmp/wrong.txt"));
        assert!(!has_audit_entry_for_rename(&entries, "/tmp/wrong.txt", "/tmp/new1.txt"));
    }

    #[test]
    fn test_read_repo_audit_log_nonexistent() {
        let temp_dir = TempDir::new().unwrap();
        let repo_root = temp_dir.path();

        let entries = read_repo_audit_log(repo_root).unwrap();
        assert!(entries.is_empty());
    }

    #[test]
    fn test_read_repo_audit_log_with_entries() {
        let temp_dir = TempDir::new().unwrap();
        let repo_root = temp_dir.path();
        let audit_path = repo_audit_log_path(repo_root);

        // Create audit log with some entries
        fs::create_dir_all(audit_path.parent().unwrap()).unwrap();
        let entry1 = crate::audit::AuditEntry::new_delete("/tmp/file1.txt".to_string(), None, None, None, None, None);
        let entry2 = crate::audit::AuditEntry::new_move("/tmp/old.txt".to_string(), "/tmp/new.txt".to_string(), None, None, None, None);

        fs::write(&audit_path, format!("{}\n{}", entry1.to_jsonl().unwrap(), entry2.to_jsonl().unwrap())).unwrap();

        let entries = read_repo_audit_log(repo_root).unwrap();
        assert_eq!(entries.len(), 2);
    }

    #[test]
    fn test_detect_staged_changes_no_changes() {
        let temp_dir = TempDir::new().unwrap();
        let repo_root = temp_dir.path();

        // Initialize a git repo
        std::process::Command::new("git")
            .args(["init"])
            .current_dir(repo_root)
            .output()
            .unwrap();

        let changes = detect_staged_changes(repo_root).unwrap();
        assert!(changes.is_empty());
    }

    #[test]
    fn test_detect_staged_changes_with_deletion() {
        let temp_dir = TempDir::new().unwrap();
        let repo_root = temp_dir.path();

        // Initialize a git repo
        std::process::Command::new("git")
            .args(["init"])
            .current_dir(repo_root)
            .output()
            .unwrap();

        // Create and commit a file
        let test_file = repo_root.join("test.txt");
        fs::write(&test_file, "content").unwrap();
        std::process::Command::new("git")
            .args(["add", "test.txt"])
            .current_dir(repo_root)
            .output()
            .unwrap();
        std::process::Command::new("git")
            .args(["config", "user.email", "test@example.com"])
            .current_dir(repo_root)
            .output()
            .unwrap();
        std::process::Command::new("git")
            .args(["config", "user.name", "Test User"])
            .current_dir(repo_root)
            .output()
            .unwrap();
        std::process::Command::new("git")
            .args(["commit", "-m", "initial"])
            .current_dir(repo_root)
            .output()
            .unwrap();

        // Delete the file and stage the deletion
        fs::remove_file(&test_file).unwrap();
        std::process::Command::new("git")
            .args(["add", "test.txt"])
            .current_dir(repo_root)
            .output()
            .unwrap();

        let changes = detect_staged_changes(repo_root).unwrap();
        assert_eq!(changes.len(), 1);
        match &changes[0] {
            GitChange::Deletion { path } => assert_eq!(path, "test.txt"),
            _ => panic!("Expected deletion"),
        }
    }

    #[test]
    fn test_run_pre_commit_hook_with_raw_deletion() {
        let temp_dir = TempDir::new().unwrap();
        let repo_root = temp_dir.path();

        // Initialize a git repo
        std::process::Command::new("git")
            .args(["init"])
            .current_dir(repo_root)
            .output()
            .unwrap();

        // Create and commit a file
        let test_file = repo_root.join("test.txt");
        fs::write(&test_file, "content").unwrap();
        std::process::Command::new("git")
            .args(["add", "test.txt"])
            .current_dir(repo_root)
            .output()
            .unwrap();
        std::process::Command::new("git")
            .args(["config", "user.email", "test@example.com"])
            .current_dir(repo_root)
            .output()
            .unwrap();
        std::process::Command::new("git")
            .args(["config", "user.name", "Test User"])
            .current_dir(repo_root)
            .output()
            .unwrap();
        std::process::Command::new("git")
            .args(["commit", "-m", "initial"])
            .current_dir(repo_root)
            .output()
            .unwrap();

        // Delete the file and stage the deletion (no audit entry)
        fs::remove_file(&test_file).unwrap();
        std::process::Command::new("git")
            .args(["add", "test.txt"])
            .current_dir(repo_root)
            .output()
            .unwrap();

        // Hook should fail due to raw deletion
        let result = run_pre_commit_hook(repo_root);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Raw deletion detected"));
    }

    #[test]
    fn test_run_pre_commit_hook_with_audit_entry() {
        let temp_dir = TempDir::new().unwrap();
        let repo_root = temp_dir.path();

        // Initialize a git repo
        std::process::Command::new("git")
            .args(["init"])
            .current_dir(repo_root)
            .output()
            .unwrap();

        // Create and commit a file
        let test_file = repo_root.join("test.txt");
        fs::write(&test_file, "content").unwrap();
        std::process::Command::new("git")
            .args(["add", "test.txt"])
            .current_dir(repo_root)
            .output()
            .unwrap();
        std::process::Command::new("git")
            .args(["config", "user.email", "test@example.com"])
            .current_dir(repo_root)
            .output()
            .unwrap();
        std::process::Command::new("git")
            .args(["config", "user.name", "Test User"])
            .current_dir(repo_root)
            .output()
            .unwrap();
        std::process::Command::new("git")
            .args(["commit", "-m", "initial"])
            .current_dir(repo_root)
            .output()
            .unwrap();

        // Create audit log with deletion entry (use relative path to match git diff output)
        let audit_path = repo_audit_log_path(repo_root);
        fs::create_dir_all(audit_path.parent().unwrap()).unwrap();
        let entry = crate::audit::AuditEntry::new_delete(
            "test.txt".to_string(),  // Use relative path
            None,
            None,
            None,
            None,
            None,
        );
        fs::write(&audit_path, entry.to_jsonl().unwrap()).unwrap();

        // Delete the file and stage the deletion (has audit entry)
        fs::remove_file(&test_file).unwrap();
        std::process::Command::new("git")
            .args(["add", "test.txt"])
            .current_dir(repo_root)
            .output()
            .unwrap();

        // Hook should pass
        let result = run_pre_commit_hook(repo_root);
        assert!(result.is_ok());
    }
}
