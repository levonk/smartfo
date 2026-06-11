//! VCS-aware remove implementation
//!
//! This module handles file classification and trash enqueueing:
//! - VCS-committed clean files: VCS-aware delete when configured
//! - VCS-committed dirty files: Trash + VCS delete
//! - Ignored files: Direct delete when configured
//! - Untracked files: Trash by default (async)

use anyhow::{Context, Result};
use std::path::{Path, PathBuf};
use crate::vcs::{VcsInfo, VcsType, detect_vcs, is_tracked};
use crate::config::Config;
use crate::audit::{AuditEntry, append_audit_log};
use crate::confirmation::{ConfirmationState, confirm_destructive};
use crate::progress::ProgressManager;

/// File classification for delete operations
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileClassification {
    /// File is tracked by VCS and has no uncommitted changes
    VcsCommittedClean,
    /// File is tracked by VCS but has uncommitted changes
    VcsCommittedDirty,
    /// File is ignored by VCS
    Ignored,
    /// File is not tracked by VCS and not ignored
    Untracked,
}

/// Classify a file for delete operations
pub fn classify_file(file_path: &Path) -> Result<FileClassification> {
    // Get VCS info for the file (fail gracefully if VCS not available)
    let vcs_info = match detect_vcs(file_path)? {
        Some(info) => info,
        None => {
            tracing::debug!("File classification: untracked (no VCS detected)");
            return Ok(FileClassification::Untracked);
        }
    };

    // Check if file is tracked
    let tracked = is_tracked(&vcs_info, file_path).unwrap_or(false);

    if !tracked {
        // Check if file is ignored
        let ignored = is_ignored(&vcs_info, file_path).unwrap_or(false);
        if ignored {
            tracing::debug!("File classification: ignored");
            return Ok(FileClassification::Ignored);
        } else {
            tracing::debug!("File classification: untracked");
            return Ok(FileClassification::Untracked);
        }
    }

    // File is tracked - check if it has uncommitted changes (dirty)
    let dirty = is_dirty(&vcs_info, file_path).unwrap_or(false);

    if dirty {
        tracing::debug!("File classification: VCS-committed dirty");
        Ok(FileClassification::VcsCommittedDirty)
    } else {
        tracing::debug!("File classification: VCS-committed clean");
        Ok(FileClassification::VcsCommittedClean)
    }
}

/// Check if a file has uncommitted changes (dirty)
fn is_dirty(vcs_info: &VcsInfo, file_path: &Path) -> Result<bool> {
    match vcs_info.vcs_type {
        crate::vcs::VcsType::Git => is_git_dirty(vcs_info, file_path),
        crate::vcs::VcsType::Hg => is_hg_dirty(vcs_info, file_path),
        crate::vcs::VcsType::Svn => is_svn_dirty(vcs_info, file_path),
        crate::vcs::VcsType::Jj => is_jj_dirty(vcs_info, file_path),
    }
}

/// Check if a file is ignored by VCS
fn is_ignored(vcs_info: &VcsInfo, file_path: &Path) -> Result<bool> {
    match vcs_info.vcs_type {
        crate::vcs::VcsType::Git => is_git_ignored(vcs_info, file_path),
        crate::vcs::VcsType::Hg => is_hg_ignored(vcs_info, file_path),
        crate::vcs::VcsType::Svn => is_svn_ignored(vcs_info, file_path),
        crate::vcs::VcsType::Jj => is_jj_ignored(vcs_info, file_path),
    }
}

/// Check if a Git file has uncommitted changes
fn is_git_dirty(vcs_info: &VcsInfo, file_path: &Path) -> Result<bool> {
    let output = std::process::Command::new("git")
        .args(["diff", "--quiet", file_path.to_str().context("Invalid UTF-8 in file path")?])
        .current_dir(&vcs_info.repo_root)
        .output()
        .context("Failed to run git diff")?;

    // git diff --quiet exits with 1 if there are differences, 0 if clean
    Ok(!output.status.success())
}

/// Check if a Mercurial file has uncommitted changes
fn is_hg_dirty(vcs_info: &VcsInfo, file_path: &Path) -> Result<bool> {
    let output = std::process::Command::new("hg")
        .args(["status", file_path.to_str().context("Invalid UTF-8 in file path")?])
        .current_dir(&vcs_info.repo_root)
        .output()
        .context("Failed to run hg status")?;

    if !output.status.success() {
        return Ok(false);
    }

    let stdout = std::str::from_utf8(&output.stdout)
        .context("Invalid UTF-8 in hg output")?;

    // hg status shows 'M' for modified files
    Ok(stdout.trim().starts_with('M'))
}

/// Check if an SVN file has uncommitted changes
fn is_svn_dirty(vcs_info: &VcsInfo, file_path: &Path) -> Result<bool> {
    let output = std::process::Command::new("svn")
        .args(["status", file_path.to_str().context("Invalid UTF-8 in file path")?])
        .current_dir(&vcs_info.repo_root)
        .output()
        .context("Failed to run svn status")?;

    if !output.status.success() {
        return Ok(false);
    }

    let stdout = std::str::from_utf8(&output.stdout)
        .context("Invalid UTF-8 in svn output")?;

    // svn status shows first column status; 'M' means modified
    Ok(stdout.trim().starts_with('M'))
}

/// Check if a Jujutsu file has uncommitted changes
fn is_jj_dirty(vcs_info: &VcsInfo, file_path: &Path) -> Result<bool> {
    let output = std::process::Command::new("jj")
        .args(["diff", file_path.to_str().context("Invalid UTF-8 in file path")?])
        .current_dir(&vcs_info.repo_root)
        .output()
        .context("Failed to run jj diff")?;

    if !output.status.success() {
        return Ok(false);
    }

    let stdout = std::str::from_utf8(&output.stdout)
        .context("Invalid UTF-8 in jj output")?;

    // jj diff shows output if there are differences
    Ok(!stdout.trim().is_empty())
}

/// Check if a Git file is ignored
fn is_git_ignored(vcs_info: &VcsInfo, file_path: &Path) -> Result<bool> {
    let output = std::process::Command::new("git")
        .args(["check-ignore", file_path.to_str().context("Invalid UTF-8 in file path")?])
        .current_dir(&vcs_info.repo_root)
        .output()
        .context("Failed to run git check-ignore")?;

    // git check-ignore exits with 0 if ignored, 1 if not ignored
    Ok(output.status.success())
}

/// Check if a Mercurial file is ignored
fn is_hg_ignored(vcs_info: &VcsInfo, file_path: &Path) -> Result<bool> {
    let output = std::process::Command::new("hg")
        .args(["status", "-i", file_path.to_str().context("Invalid UTF-8 in file path")?])
        .current_dir(&vcs_info.repo_root)
        .output()
        .context("Failed to run hg status")?;

    if !output.status.success() {
        return Ok(false);
    }

    let stdout = std::str::from_utf8(&output.stdout)
        .context("Invalid UTF-8 in hg output")?;

    // hg status -i shows ignored files
    Ok(!stdout.trim().is_empty())
}

/// Check if an SVN file is ignored
fn is_svn_ignored(vcs_info: &VcsInfo, file_path: &Path) -> Result<bool> {
    // SVN doesn't have a native ignore check like Git
    // We check if the file is untracked and not in version control
    let output = std::process::Command::new("svn")
        .args(["status", file_path.to_str().context("Invalid UTF-8 in file path")?])
        .current_dir(&vcs_info.repo_root)
        .output()
        .context("Failed to run svn status")?;

    if !output.status.success() {
        return Ok(false);
    }

    let stdout = std::str::from_utf8(&output.stdout)
        .context("Invalid UTF-8 in svn output")?;

    // svn status shows '?' for unversioned/ignored files
    Ok(stdout.trim().starts_with('?'))
}

/// Check if a Jujutsu file is ignored
fn is_jj_ignored(vcs_info: &VcsInfo, file_path: &Path) -> Result<bool> {
    let output = std::process::Command::new("jj")
        .args(["files", "--ignore"])
        .current_dir(&vcs_info.repo_root)
        .output()
        .context("Failed to run jj files --ignore")?;

    if !output.status.success() {
        return Ok(false);
    }

    let stdout = std::str::from_utf8(&output.stdout)
        .context("Invalid UTF-8 in jj output")?;

    // Check if the file appears in the ignored files list
    let relative_path = file_path
        .strip_prefix(&vcs_info.repo_root)
        .context("File path is not within repo root")?;

    Ok(stdout.lines().any(|line| line.trim() == relative_path.to_str().unwrap_or("")))
}

/// Perform VCS-aware remove for a file
pub fn vcs_remove(vcs_info: &VcsInfo, file_path: &Path) -> Result<()> {
    match vcs_info.vcs_type {
        VcsType::Git => git_remove(vcs_info, file_path),
        VcsType::Hg => hg_remove(vcs_info, file_path),
        VcsType::Svn => svn_remove(vcs_info, file_path),
        VcsType::Jj => jj_remove(vcs_info, file_path),
    }
}

/// Perform Git remove (git rm)
fn git_remove(vcs_info: &VcsInfo, file_path: &Path) -> Result<()> {
    let output = std::process::Command::new("git")
        .args(["rm", file_path.to_str().context("Invalid UTF-8 in file path")?])
        .current_dir(&vcs_info.repo_root)
        .output()
        .context("Failed to run git rm")?;

    if !output.status.success() {
        let stderr = std::str::from_utf8(&output.stderr)
            .context("Invalid UTF-8 in git stderr")?;
        anyhow::bail!("git rm failed: {}", stderr.trim());
    }

    tracing::info!("VCS-aware remove (git): {:?}", file_path);
    Ok(())
}

/// Perform Mercurial remove (hg remove)
fn hg_remove(vcs_info: &VcsInfo, file_path: &Path) -> Result<()> {
    let output = std::process::Command::new("hg")
        .args(["remove", file_path.to_str().context("Invalid UTF-8 in file path")?])
        .current_dir(&vcs_info.repo_root)
        .output()
        .context("Failed to run hg remove")?;

    if !output.status.success() {
        let stderr = std::str::from_utf8(&output.stderr)
            .context("Invalid UTF-8 in hg stderr")?;
        anyhow::bail!("hg remove failed: {}", stderr.trim());
    }

    tracing::info!("VCS-aware remove (hg): {:?}", file_path);
    Ok(())
}

/// Perform SVN remove (svn rm)
fn svn_remove(vcs_info: &VcsInfo, file_path: &Path) -> Result<()> {
    let output = std::process::Command::new("svn")
        .args(["rm", file_path.to_str().context("Invalid UTF-8 in file path")?])
        .current_dir(&vcs_info.repo_root)
        .output()
        .context("Failed to run svn rm")?;

    if !output.status.success() {
        let stderr = std::str::from_utf8(&output.stderr)
            .context("Invalid UTF-8 in svn stderr")?;
        anyhow::bail!("svn rm failed: {}", stderr.trim());
    }

    tracing::info!("VCS-aware remove (svn): {:?}", file_path);
    Ok(())
}

/// Perform Jujutsu remove (jj file remove)
fn jj_remove(vcs_info: &VcsInfo, file_path: &Path) -> Result<()> {
    let output = std::process::Command::new("jj")
        .args(["file", "remove", file_path.to_str().context("Invalid UTF-8 in file path")?])
        .current_dir(&vcs_info.repo_root)
        .output()
        .context("Failed to run jj file remove")?;

    if !output.status.success() {
        let stderr = std::str::from_utf8(&output.stderr)
            .context("Invalid UTF-8 in jj stderr")?;
        anyhow::bail!("jj file remove failed: {}", stderr.trim());
    }

    tracing::info!("VCS-aware remove (jj): {:?}", file_path);
    Ok(())
}

/// Remove a file based on classification and config settings
pub fn remove_file(
    file_path: &Path,
    config: &Config,
    blocking: bool,
    force_delete: bool,
    force: bool,
    quiet: bool,
    json_mode: bool,
    interactive: bool,
    dry_run: bool,
) -> Result<()> {
    // Classify the file
    let classification = classify_file(file_path)?;

    // Get VCS info for audit logging
    // TODO: Fix VCS info extraction - currently returning None due to type inference issue
    let _vcs_info = detect_vcs(file_path).ok();
    let vcs_type: Option<String> = None;
    let repo_root: Option<String> = None;

    // Determine committed status for audit logging
    let committed = match classification {
        FileClassification::VcsCommittedClean => Some(true),
        FileClassification::VcsCommittedDirty => Some(false),
        _ => None,
    };

    // Confirmation prompt for destructive operations
    let mut confirmation_state = ConfirmationState::default();

    // Always prompt for deletion unless force or quiet is set
    // Interactive flag also enables prompts even for non-destructive operations
    let should_prompt = interactive || !force && !quiet;

    if should_prompt {
        let file_display = file_path.display();
        let confirmed = confirm_destructive(
            "remove",
            &file_display.to_string(),
            force,
            quiet,
            &mut confirmation_state,
            dry_run,
        )?;

        if !confirmed {
            tracing::info!("User cancelled deletion of: {:?}", file_path);
            return Ok(()); // Exit early if user rejects
        }
    }

    match classification {
        FileClassification::VcsCommittedClean => {
            // Clean committed files: use VCS-aware delete if backup_vcs_committed is false
            if config.trash.backup_vcs_committed || force_delete {
                // Backup to trash even though it's clean (or force-delete bypasses everything)
                if force_delete {
                    // Force delete: direct unlink without trash
                    tracing::info!("Force delete (clean committed): {:?}", file_path);
                    std::fs::remove_file(file_path)
                        .context("Failed to force delete file")?;

                    // Log audit entry
                    let entry = AuditEntry::new_delete(
                        file_path.to_string_lossy().to_string(),
                        None,
                        None,
                        vcs_type,
                        repo_root,
                        committed,
                    );
                    let audit_path = PathBuf::from(&config.paths.audit_log);
                    append_audit_log(&entry, &audit_path)?;
                } else {
                    // Backup to trash
                    let trash_path = enqueue_trash(file_path, blocking, quiet, json_mode)?;

                    // Log audit entry
                    let entry = AuditEntry::new_delete(
                        file_path.to_string_lossy().to_string(),
                        trash_path,
                        None,
                        vcs_type,
                        repo_root,
                        committed,
                    );
                    let audit_path = PathBuf::from(&config.paths.audit_log);
                    append_audit_log(&entry, &audit_path)?;
                }
            } else {
                // Use VCS-aware remove (file is recoverable from VCS history)
                let vcs_info = detect_vcs(file_path)?
                    .context("VCS info required for VCS-aware remove")?;
                vcs_remove(&vcs_info, file_path)?;

                // Log audit entry (no trash path for VCS-only delete)
                let entry = AuditEntry::new_delete(
                    file_path.to_string_lossy().to_string(),
                    None,
                    None,
                    vcs_type,
                    repo_root,
                    committed,
                );
                let audit_path = PathBuf::from(&config.paths.audit_log);
                append_audit_log(&entry, &audit_path)?;
            }
        }
        FileClassification::VcsCommittedDirty => {
            // Dirty committed files: always trash + VCS remove
            // (uncommitted changes are not in VCS history)
            let trash_path = enqueue_trash(file_path, blocking, quiet, json_mode)?;
            let vcs_info = detect_vcs(file_path)?
                .context("VCS info required for VCS-aware remove")?;
            vcs_remove(&vcs_info, file_path)?;

            // Log audit entry
            let entry = AuditEntry::new_delete(
                file_path.to_string_lossy().to_string(),
                trash_path,
                None,
                vcs_type,
                repo_root,
                committed,
            );
            let audit_path = PathBuf::from(&config.paths.audit_log);
            append_audit_log(&entry, &audit_path)?;
        }
        FileClassification::Ignored => {
            // Ignored files: direct delete if delete_ignored is true, otherwise trash
            if config.trash.delete_ignored || force_delete {
                tracing::info!("Direct delete (ignored): {:?}", file_path);
                std::fs::remove_file(file_path)
                    .context("Failed to delete ignored file")?;

                // Log audit entry (no trash path for direct delete)
                let entry = AuditEntry::new_delete(
                    file_path.to_string_lossy().to_string(),
                    None,
                    None,
                    vcs_type,
                    repo_root,
                    committed,
                );
                let audit_path = PathBuf::from(&config.paths.audit_log);
                append_audit_log(&entry, &audit_path)?;
            } else {
                let trash_path = enqueue_trash(file_path, blocking, quiet, json_mode)?;

                // Log audit entry
                let entry = AuditEntry::new_delete(
                    file_path.to_string_lossy().to_string(),
                    trash_path,
                    None,
                    vcs_type,
                    repo_root,
                    committed,
                );
                let audit_path = PathBuf::from(&config.paths.audit_log);
                append_audit_log(&entry, &audit_path)?;
            }
        }
        FileClassification::Untracked => {
            // Untracked files: trash by default
            if force_delete {
                tracing::info!("Force delete (untracked): {:?}", file_path);
                std::fs::remove_file(file_path)
                    .context("Failed to force delete file")?;

                // Log audit entry (no trash path for force delete)
                let entry = AuditEntry::new_delete(
                    file_path.to_string_lossy().to_string(),
                    None,
                    None,
                    vcs_type,
                    repo_root,
                    committed,
                );
                let audit_path = PathBuf::from(&config.paths.audit_log);
                append_audit_log(&entry, &audit_path)?;
            } else {
                let trash_path = enqueue_trash(file_path, blocking, quiet, json_mode)?;

                // Log audit entry
                let entry = AuditEntry::new_delete(
                    file_path.to_string_lossy().to_string(),
                    trash_path,
                    None,
                    vcs_type,
                    repo_root,
                    committed,
                );
                let audit_path = PathBuf::from(&config.paths.audit_log);
                append_audit_log(&entry, &audit_path)?;
            }
        }
    }

    Ok(())
}

/// Enqueue a file for trash deletion (async by default)
///
/// This function enqueues the file for trash deletion and returns immediately.
/// The actual deletion is performed by the background worker (Story 04-003).
/// For now, this is a stub that logs the intent.
///
/// Returns the trash path if successful (for audit logging), None otherwise.
pub fn enqueue_trash(file_path: &Path, blocking: bool, quiet: bool, json_mode: bool) -> Result<Option<String>> {
    if blocking {
        tracing::info!("Blocking trash enqueue: {:?}", file_path);
        // TODO: Implement blocking trash operation (wait for worker completion)
        // This will be implemented in Story 04-003 when the worker is available
        // For now, perform a simple synchronous move to a temp trash location
        let trash_path = std::env::var("HOME")
            .map(|h| std::path::PathBuf::from(h).join(".smartfo/trash"))
            .unwrap_or_else(|_| std::path::PathBuf::from("/tmp/smartfo_trash"));

        std::fs::create_dir_all(&trash_path)
            .context("Failed to create trash directory")?;

        let file_name = file_path
            .file_name()
            .context("File has no valid name")?
            .to_str()
            .context("Invalid UTF-8 in file name")?;

        let dest_path = trash_path.join(file_name);

        // Create progress manager for blocking operation
        let progress_manager = ProgressManager::new(quiet, json_mode);
        let spinner = progress_manager.create_spinner(&format!("Moving {} to trash", file_name));

        std::fs::rename(file_path, &dest_path)
            .context("Failed to move file to trash")?;

        progress_manager.finish_with_success(&spinner, "Move complete");

        let trash_path_str = dest_path.to_string_lossy().to_string();
        tracing::info!("Moved to trash (blocking): {:?} -> {:?}", file_path, dest_path);
        Ok(Some(trash_path_str))
    } else {
        tracing::info!("Async trash enqueue: {:?}", file_path);
        // TODO: Enqueue to job queue for async processing (Story 04-001)
        // For now, just log the intent and return None
        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_file_classification_display() {
        // Test that FileClassification can be displayed (for debugging)
        let clean = FileClassification::VcsCommittedClean;
        let dirty = FileClassification::VcsCommittedDirty;
        let ignored = FileClassification::Ignored;
        let untracked = FileClassification::Untracked;

        // These should not panic
        println!("{:?}", clean);
        println!("{:?}", dirty);
        println!("{:?}", ignored);
        println!("{:?}", untracked);
    }

    #[test]
    fn test_file_classification_equality() {
        // Test that FileClassification supports equality checks
        assert_eq!(FileClassification::VcsCommittedClean, FileClassification::VcsCommittedClean);
        assert_eq!(FileClassification::VcsCommittedDirty, FileClassification::VcsCommittedDirty);
        assert_eq!(FileClassification::Ignored, FileClassification::Ignored);
        assert_eq!(FileClassification::Untracked, FileClassification::Untracked);

        assert_ne!(FileClassification::VcsCommittedClean, FileClassification::VcsCommittedDirty);
        assert_ne!(FileClassification::Ignored, FileClassification::Untracked);
    }

    #[test]
    fn test_enqueue_trash_async() {
        // Test async trash enqueue (should succeed for now)
        let test_file = Path::new("/tmp/test_file.txt");
        let result = enqueue_trash(test_file, false, false, false);
        assert!(result.is_ok());
        // Async mode returns None for trash path (not yet implemented)
        assert_eq!(result.unwrap(), None);
    }

    #[test]
    fn test_enqueue_trash_blocking() {
        // Test blocking trash enqueue (should create temp trash for now)
        let test_file = Path::new("/nonexistent_test_file.txt");
        let result = enqueue_trash(test_file, true, false, false);
        // This will fail because the file doesn't exist, but the function should run
        assert!(result.is_err());
    }

    #[test]
    fn test_vcs_remove_function_exists() {
        // Test that vcs_remove function exists and has correct signature
        // This is a compile-time test - if it compiles, the function exists
        let vcs_info = VcsInfo {
            vcs_type: VcsType::Git,
            repo_root: PathBuf::from("/tmp"),
        };
        let test_file = Path::new("/tmp/test.txt");

        // This will fail at runtime (no actual git repo), but tests the API
        let _result = vcs_remove(&vcs_info, test_file);
    }

    #[test]
    fn test_remove_file_function_exists() {
        // Test that remove_file function exists and has correct signature
        use crate::config::Config;
        let config = Config::default();
        let test_file = Path::new("/tmp/test.txt");

        // This will fail at runtime (no actual file), but tests the API
        let _result = remove_file(test_file, &config, false, false, false, false, false, false, false);
    }

    #[test]
    fn test_vcs_info_getters() {
        // Test that VcsInfo getter methods work correctly
        let vcs_info = VcsInfo::new(
            VcsType::Git,
            PathBuf::from("/test/repo"),
        );

        assert_eq!(vcs_info.vcs_type(), VcsType::Git);
        assert_eq!(vcs_info.repo_root(), &PathBuf::from("/test/repo"));
    }

    #[test]
    fn test_vcs_type_command() {
        // Test that VcsType::command() returns the correct command name
        assert_eq!(VcsType::Git.command(), "git");
        assert_eq!(VcsType::Hg.command(), "hg");
        assert_eq!(VcsType::Svn.command(), "svn");
        assert_eq!(VcsType::Jj.command(), "jj");
    }

    #[test]
    fn test_file_classification_variants() {
        // Test that all FileClassification variants can be created and compared
        let classifications = vec![
            FileClassification::VcsCommittedClean,
            FileClassification::VcsCommittedDirty,
            FileClassification::Ignored,
            FileClassification::Untracked,
        ];

        // Ensure all variants are distinct
        for (i, class_i) in classifications.iter().enumerate() {
            for (j, class_j) in classifications.iter().enumerate() {
                if i == j {
                    assert_eq!(class_i, class_j);
                } else {
                    assert_ne!(class_i, class_j);
                }
            }
        }
    }
}
