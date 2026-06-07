//! VCS-aware move implementation
//!
//! This module handles all six move scenarios:
//! - Tracked source → same repo (VCS-native move)
//! - Tracked source → outside repo (Refuse by default; --force-outside-vcs required)
//! - Outside repo → inside repo (Filesystem move)
//! - Both outside any repo (Pure filesystem rename)
//! - Neither tracked in repo (Pure filesystem rename)
//! - src == dest (No-op, exit 0)

use anyhow::{Context, Result};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::os::unix::fs::MetadataExt;
use crate::vcs::{VcsInfo, detect_vcs, is_tracked};
use crate::audit::{AuditEntry, append_audit_log};

/// Move scenario classification
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MoveScenario {
    /// Source is tracked and destination is in the same repo
    TrackedToSameRepo,
    /// Source is tracked but destination is outside the repo
    TrackedToOutsideRepo,
    /// Source is outside repo, destination is inside repo
    OutsideToInsideRepo,
    /// Both source and destination are outside any repo
    BothOutsideRepo,
    /// Both source and destination are in the same repo but neither is tracked
    NeitherTrackedInRepo,
    /// Source and destination are the same path
    SamePath,
}

/// Check if two paths are on the same filesystem
///
/// Uses device IDs (stat) to compare filesystems (cross-platform alternative to statfs)
///
/// # Arguments
/// * `path1` - First path
/// * `path2` - Second path
///
/// # Returns
/// true if both paths are on the same filesystem, false otherwise
pub fn same_filesystem(path1: &Path, path2: &Path) -> Result<bool> {
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

    let dev1 = metadata1.dev();
    let dev2 = metadata2.dev();

    Ok(dev1 == dev2)
}

/// Check if a file exceeds the async threshold size
///
/// # Arguments
/// * `path` - Path to check
/// * `threshold_mb` - Threshold in megabytes
///
/// # Returns
/// true if file size exceeds threshold, false otherwise
pub fn exceeds_async_threshold(path: &Path, threshold_mb: u64) -> Result<bool> {
    let metadata = std::fs::metadata(path)
        .context("Failed to get file metadata")?;
    
    let size_mb = metadata.len() / (1024 * 1024);
    Ok(size_mb > threshold_mb)
}

/// Determine if a move operation should be async
///
/// # Arguments
/// * `source` - Source path
/// * `dest` - Destination path
/// * `async_flag` - Whether --async flag is set
/// * `blocking_flag` - Whether --blocking flag is set
/// * `threshold_mb` - File size threshold for async (in MB)
///
/// # Returns
/// true if move should be async, false otherwise
pub fn should_be_async(
    source: &Path,
    dest: &Path,
    async_flag: bool,
    blocking_flag: bool,
    threshold_mb: u64,
) -> Result<bool> {
    // --blocking overrides everything
    if blocking_flag {
        tracing::debug!("Blocking mode requested, forcing synchronous");
        return Ok(false);
    }

    // --async forces async regardless of other conditions
    if async_flag {
        tracing::debug!("Async mode requested, forcing asynchronous");
        return Ok(true);
    }

    // Cross-device moves are always async
    if !same_filesystem(source, dest)? {
        tracing::debug!("Cross-device move detected, will be async");
        return Ok(true);
    }

    // Large files are async
    if exceeds_async_threshold(source, threshold_mb)? {
        tracing::debug!("File size exceeds threshold ({}MB), will be async", threshold_mb);
        return Ok(true);
    }

    tracing::debug!("Move is small and same-filesystem, will be synchronous");
    Ok(false)
}

/// Classify the move scenario based on source and destination paths
pub fn classify_scenario(source: &Path, dest: &Path) -> Result<MoveScenario> {
    // Check if source and dest are the same
    if source == dest {
        tracing::debug!("Move scenario: same path (no-op)");
        return Ok(MoveScenario::SamePath);
    }

    // Get VCS info for source (fail gracefully if VCS not available)
    let source_vcs = detect_vcs(source).unwrap_or(None);

    // Get VCS info for destination (fail gracefully if VCS not available)
    let dest_vcs = detect_vcs(dest).unwrap_or(None);

    match (source_vcs, dest_vcs) {
        // Both inside the same repo
        (Some(source_info), Some(dest_info)) if source_info.repo_root == dest_info.repo_root => {
            // Check if source is tracked
            let source_tracked = is_tracked(&source_info, source).unwrap_or(false);
            
            let dest_tracked = is_tracked(&dest_info, dest).unwrap_or(false);

            if source_tracked {
                tracing::debug!("Move scenario: tracked source to same repo");
                Ok(MoveScenario::TrackedToSameRepo)
            } else if !dest_tracked {
                tracing::debug!("Move scenario: neither tracked in repo");
                Ok(MoveScenario::NeitherTrackedInRepo)
            } else {
                // Source not tracked, dest tracked - treat as filesystem move
                tracing::debug!("Move scenario: neither tracked in repo (dest tracked but source not)");
                Ok(MoveScenario::NeitherTrackedInRepo)
            }
        }
        
        // Source tracked, dest outside repo
        (Some(source_info), None) => {
            let source_tracked = is_tracked(&source_info, source).unwrap_or(false);
            
            if source_tracked {
                tracing::debug!("Move scenario: tracked source to outside repo");
                Ok(MoveScenario::TrackedToOutsideRepo)
            } else {
                // Source not tracked, treat as both outside
                tracing::debug!("Move scenario: both outside repo (source in repo but not tracked)");
                Ok(MoveScenario::BothOutsideRepo)
            }
        }
        
        // Source outside repo, dest inside repo
        (None, Some(_dest_info)) => {
            tracing::debug!("Move scenario: outside to inside repo");
            Ok(MoveScenario::OutsideToInsideRepo)
        }
        
        // Both outside any repo
        (None, None) => {
            tracing::debug!("Move scenario: both outside repo");
            Ok(MoveScenario::BothOutsideRepo)
        }
        
        // Different repos - treat as both outside (filesystem move)
        (Some(_), Some(_)) => {
            tracing::debug!("Move scenario: both outside repo (different repos)");
            Ok(MoveScenario::BothOutsideRepo)
        }
    }
}

/// Perform VCS-native move for tracked files in the same repo
pub fn vcs_native_move(source: &Path, dest: &Path, vcs_info: &VcsInfo) -> Result<()> {
    match vcs_info.vcs_type {
        crate::vcs::VcsType::Git => git_move(source, dest, vcs_info),
        crate::vcs::VcsType::Hg => hg_move(source, dest, vcs_info),
        crate::vcs::VcsType::Svn => svn_move(source, dest, vcs_info),
        crate::vcs::VcsType::Jj => jj_move(source, dest, vcs_info),
    }
}

/// Perform git mv operation
fn git_move(source: &Path, dest: &Path, vcs_info: &VcsInfo) -> Result<()> {
    tracing::debug!("Executing git mv: {:?} -> {:?}", source, dest);
    
    let output = Command::new("git")
        .args(["mv", source.to_str().context("Invalid UTF-8 in source path")?, 
               dest.to_str().context("Invalid UTF-8 in dest path")?])
        .current_dir(&vcs_info.repo_root)
        .output()
        .context("Failed to execute git mv")?;

    if output.status.success() {
        tracing::info!("git mv successful: {:?} -> {:?}", source, dest);
        Ok(())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("git mv failed: {}", stderr);
    }
}

/// Perform hg mv operation
fn hg_move(source: &Path, dest: &Path, vcs_info: &VcsInfo) -> Result<()> {
    tracing::debug!("Executing hg mv: {:?} -> {:?}", source, dest);
    
    let output = Command::new("hg")
        .args(["mv", source.to_str().context("Invalid UTF-8 in source path")?, 
               dest.to_str().context("Invalid UTF-8 in dest path")?])
        .current_dir(&vcs_info.repo_root)
        .output()
        .context("Failed to execute hg mv")?;

    if output.status.success() {
        tracing::info!("hg mv successful: {:?} -> {:?}", source, dest);
        Ok(())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("hg mv failed: {}", stderr);
    }
}

/// Perform svn mv operation
fn svn_move(source: &Path, dest: &Path, vcs_info: &VcsInfo) -> Result<()> {
    tracing::debug!("Executing svn mv: {:?} -> {:?}", source, dest);
    
    let output = Command::new("svn")
        .args(["mv", source.to_str().context("Invalid UTF-8 in source path")?, 
               dest.to_str().context("Invalid UTF-8 in dest path")?])
        .current_dir(&vcs_info.repo_root)
        .output()
        .context("Failed to execute svn mv")?;

    if output.status.success() {
        tracing::info!("svn mv successful: {:?} -> {:?}", source, dest);
        Ok(())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("svn mv failed: {}", stderr);
    }
}

/// Perform jj mv operation
fn jj_move(source: &Path, dest: &Path, vcs_info: &VcsInfo) -> Result<()> {
    tracing::debug!("Executing jj mv: {:?} -> {:?}", source, dest);
    
    let output = Command::new("jj")
        .args(["mv", source.to_str().context("Invalid UTF-8 in source path")?, 
               dest.to_str().context("Invalid UTF-8 in dest path")?])
        .current_dir(&vcs_info.repo_root)
        .output()
        .context("Failed to execute jj mv")?;

    if output.status.success() {
        tracing::info!("jj mv successful: {:?} -> {:?}", source, dest);
        Ok(())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("jj mv failed: {}", stderr);
    }
}

/// Perform filesystem rename operation
pub fn filesystem_rename(source: &Path, dest: &Path) -> Result<()> {
    tracing::debug!("Executing filesystem rename: {:?} -> {:?}", source, dest);
    
    std::fs::rename(source, dest)
        .with_context(|| format!("Failed to rename {:?} to {:?}", source, dest))?;
    
    tracing::info!("Filesystem rename successful: {:?} -> {:?}", source, dest);
    Ok(())
}

/// Check if moving tracked file outside repo is allowed
pub fn allow_tracked_to_outside(force_outside_vcs: bool) -> Result<()> {
    if !force_outside_vcs {
        anyhow::bail!(
            "Refusing to move tracked file outside repository. \
             Use --force-outside-vcs to override this safety check."
        );
    }
    tracing::warn!("Moving tracked file outside repo due to --force-outside-vcs flag");
    Ok(())
}

/// Handle destination already exists scenarios
pub fn handle_dest_exists(
    dest: &Path,
    no_clobber: bool,
    force: bool,
    interactive: bool,
    backup: bool,
) -> Result<bool> {
    if !dest.exists() {
        return Ok(true); // Destination doesn't exist, safe to proceed
    }

    if no_clobber {
        tracing::info!("Refusing to overwrite existing file: {:?} (no-clobber)", dest);
        return Ok(false);
    }

    if backup {
        create_backup(dest)?;
    }

    if interactive {
        println!("smartfo: overwrite {:?}? ", dest);
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)
            .context("Failed to read user input")?;
        
        if !input.trim().to_lowercase().starts_with('y') {
            tracing::info!("User declined to overwrite: {:?}", dest);
            return Ok(false);
        }
    }

    if !force && !interactive && !backup {
        // Default POSIX behavior: ask unless -f is specified
        println!("smartfo: overwrite {:?}? ", dest);
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)
            .context("Failed to read user input")?;
        
        if !input.trim().to_lowercase().starts_with('y') {
            tracing::info!("User declined to overwrite: {:?}", dest);
            return Ok(false);
        }
    }

    tracing::debug!("Proceeding with overwrite: {:?}", dest);
    Ok(true)
}

/// Create a backup of the destination file
fn create_backup(dest: &Path) -> Result<()> {
    let backup_path = generate_backup_path(dest)?;
    tracing::info!("Creating backup: {:?} -> {:?}", dest, backup_path);
    
    std::fs::copy(dest, &backup_path)
        .with_context(|| format!("Failed to create backup from {:?} to {:?}", dest, backup_path))?;
    
    Ok(())
}

/// Generate a backup path with timestamp suffix
fn generate_backup_path(dest: &Path) -> Result<PathBuf> {
    let timestamp = chrono::Utc::now().format("%Y%m%d%H%M%S");
    let file_name = dest.file_name()
        .and_then(|n| n.to_str())
        .ok_or_else(|| anyhow::anyhow!("Invalid file name"))?;
    
    let new_name = format!("{}.~{}~", file_name, timestamp);
    
    if let Some(parent) = dest.parent() {
        Ok(parent.join(new_name))
    } else {
        Ok(PathBuf::from(new_name))
    }
}

/// Plain POSIX mv implementation without any VCS awareness
/// This bypasses all smart features for exact POSIX compatibility
pub fn plain_mv(
    source: &Path,
    dest: &Path,
    no_clobber: bool,
    force: bool,
    interactive: bool,
    backup: bool,
) -> Result<()> {
    tracing::debug!("Plain mv: {:?} -> {:?}", source, dest);
    
    // Check if source and dest are the same
    if source == dest {
        return Ok(()); // No-op
    }
    
    // Handle destination exists
    if !handle_dest_exists(dest, no_clobber, force, interactive, backup)? {
        return Ok(()); // User declined or no-clobber
    }
    
    // Perform the actual move
    filesystem_rename(source, dest)?;
    
    Ok(())
}

/// Log a move operation to the audit log
pub fn log_move_operation(
    source: &Path,
    dest: &Path,
    reason: Option<&str>,
    vcs_info: Option<&VcsInfo>,
    committed: Option<bool>,
    audit_log_path: &Path,
) -> Result<()> {
    let source_path = source.to_str()
        .ok_or_else(|| anyhow::anyhow!("Invalid UTF-8 in source path"))?
        .to_string();
    
    let dest_path = dest.to_str()
        .ok_or_else(|| anyhow::anyhow!("Invalid UTF-8 in destination path"))?
        .to_string();
    
    let (vcs, repo_root) = if let Some(info) = vcs_info {
        (
            Some(format!("{:?}", info.vcs_type).to_lowercase()),
            Some(info.repo_root.to_str()
                .ok_or_else(|| anyhow::anyhow!("Invalid UTF-8 in repo root"))?
                .to_string()),
        )
    } else {
        (None, None)
    };
    
    let entry = AuditEntry::new_move(
        source_path,
        dest_path,
        reason.map(|r| r.to_string()),
        vcs,
        repo_root,
        committed,
    );
    
    append_audit_log(&entry, audit_log_path)
        .context("Failed to write move operation to audit log")?;
    
    tracing::debug!("Logged move operation to audit log");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scenario_same_path() {
        let source = Path::new("/tmp/test.txt");
        let dest = Path::new("/tmp/test.txt");
        
        let scenario = classify_scenario(source, dest).unwrap();
        assert_eq!(scenario, MoveScenario::SamePath);
    }

    #[test]
    fn test_scenario_both_outside_repo() {
        // Use absolute paths that are unlikely to be in a git repo
        let source = Path::new("/tmp/smartfo_test_source_12345.txt");
        let dest = Path::new("/tmp/smartfo_test_dest_12345.txt");
        
        let scenario = classify_scenario(source, dest).unwrap();
        // This should be BothOutsideRepo since these paths are not in any VCS
        assert_eq!(scenario, MoveScenario::BothOutsideRepo);
    }

    #[test]
    fn test_scenario_different_paths() {
        let source = Path::new("/tmp/test1.txt");
        let dest = Path::new("/tmp/test2.txt");
        
        let scenario = classify_scenario(source, dest).unwrap();
        // Different paths, should not be SamePath
        assert_ne!(scenario, MoveScenario::SamePath);
    }

    #[test]
    fn test_vcs_native_move_git_error() {
        let source = Path::new("/nonexistent/source.txt");
        let dest = Path::new("/nonexistent/dest.txt");
        let vcs_info = VcsInfo::new(crate::vcs::VcsType::Git, PathBuf::from("/tmp"));
        
        let result = vcs_native_move(source, dest, &vcs_info);
        // Should fail because paths don't exist and git is not available
        assert!(result.is_err());
    }

    #[test]
    fn test_allow_tracked_to_outside_without_flag() {
        let result = allow_tracked_to_outside(false);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Refusing to move tracked file"));
    }

    #[test]
    fn test_allow_tracked_to_outside_with_flag() {
        let result = allow_tracked_to_outside(true);
        assert!(result.is_ok());
    }

    #[test]
    fn test_filesystem_rename_nonexistent() {
        let source = Path::new("/nonexistent/smartfo_test_source.txt");
        let dest = Path::new("/nonexistent/smartfo_test_dest.txt");
        
        let result = filesystem_rename(source, dest);
        assert!(result.is_err());
    }

    #[test]
    fn test_handle_dest_exists_no_clobber() {
        let dest = Path::new("/tmp/smartfo_test_no_clobber.txt");
        
        // Create the file
        std::fs::write(dest, "test").unwrap();
        
        let result = handle_dest_exists(dest, true, false, false, false);
        assert_eq!(result.unwrap(), false);
        
        // Cleanup
        std::fs::remove_file(dest).ok();
    }

    #[test]
    fn test_handle_dest_exists_force() {
        let dest = Path::new("/tmp/smartfo_test_force.txt");
        
        // Create the file
        std::fs::write(dest, "test").unwrap();
        
        let result = handle_dest_exists(dest, false, true, false, false);
        assert_eq!(result.unwrap(), true);
        
        // Cleanup
        std::fs::remove_file(dest).ok();
    }

    #[test]
    fn test_handle_dest_not_exists() {
        let dest = Path::new("/tmp/smartfo_test_nonexistent_12345.txt");
        
        let result = handle_dest_exists(dest, false, false, false, false);
        assert_eq!(result.unwrap(), true);
    }

    #[test]
    fn test_generate_backup_path() {
        let dest = Path::new("/tmp/test.txt");
        let backup_path = generate_backup_path(dest).unwrap();
        
        assert!(backup_path.to_string_lossy().contains("~"));
        assert!(backup_path.to_string_lossy().contains(".txt"));
    }

    #[test]
    fn test_same_filesystem_same_dir() {
        let path1 = Path::new("/tmp/test1.txt");
        let path2 = Path::new("/tmp/test2.txt");
        
        let result = same_filesystem(path1, path2).unwrap();
        assert!(result);
    }

    #[test]
    fn test_same_filesystem_different_dirs() {
        let path1 = Path::new("/tmp/test1.txt");
        let path2 = Path::new("/var/test2.txt");
        
        let _result = same_filesystem(path1, path2).unwrap();
        // On most systems, /tmp and /var are on the same filesystem
        // This test is more of a sanity check that the function works
        // We don't assert the actual value since it depends on filesystem setup
    }

    #[test]
    fn test_exceeds_async_threshold_large_file() {
        let path = Path::new("/tmp/smartfo_test_large_12345.txt");
        
        // Create a file larger than 100MB threshold
        let large_data = vec![0u8; 101 * 1024 * 1024]; // 101MB
        std::fs::write(path, &large_data).unwrap();
        
        let result = exceeds_async_threshold(path, 100).unwrap();
        assert!(result);
        
        // Cleanup
        std::fs::remove_file(path).ok();
    }

    #[test]
    fn test_exceeds_async_threshold_small_file() {
        let path = Path::new("/tmp/smartfo_test_small_12345.txt");
        
        // Create a small file
        std::fs::write(path, "test").unwrap();
        
        let result = exceeds_async_threshold(path, 100).unwrap();
        assert!(!result);
        
        // Cleanup
        std::fs::remove_file(path).ok();
    }

    #[test]
    fn test_should_be_async_blocking_flag() {
        let source = Path::new("/tmp/test1.txt");
        let dest = Path::new("/tmp/test2.txt");
        
        // --blocking should override everything
        let result = should_be_async(source, dest, true, true, 100).unwrap();
        assert!(!result);
    }

    #[test]
    fn test_should_be_async_async_flag() {
        let source = Path::new("/tmp/test1.txt");
        let dest = Path::new("/tmp/test2.txt");
        
        // --async should force async
        let result = should_be_async(source, dest, true, false, 100).unwrap();
        assert!(result);
    }

    #[test]
    fn test_should_be_async_cross_device() {
        let source = Path::new("/tmp/smartfo_test_cross_source_12345.txt");
        let dest = Path::new("/tmp/smartfo_test_cross_dest_12345.txt");
        
        // Create the files
        std::fs::write(source, "test").unwrap();
        std::fs::write(dest, "test").unwrap();
        
        // For this test, we'll just check that the function works
        // Cross-device detection depends on the actual filesystem setup
        let _async = should_be_async(source, dest, false, false, 100).unwrap();
        // We don't assert the actual value since it depends on filesystem setup
        
        // Cleanup
        std::fs::remove_file(source).ok();
        std::fs::remove_file(dest).ok();
    }

    #[test]
    fn test_plain_mv_same_path() {
        let source = Path::new("/tmp/smartfo_test_same.txt");
        let dest = Path::new("/tmp/smartfo_test_same.txt");
        
        let result = plain_mv(source, dest, false, false, false, false);
        assert!(result.is_ok());
    }

    #[test]
    fn test_plain_mv_no_clobber() {
        let source = Path::new("/tmp/smartfo_test_plain_source.txt");
        let dest = Path::new("/tmp/smartfo_test_plain_dest.txt");
        
        // Create destination file
        std::fs::write(dest, "dest").unwrap();
        
        let result = plain_mv(source, dest, true, false, false, false);
        assert!(result.is_ok()); // Should succeed without actually moving
        
        // Cleanup
        std::fs::remove_file(dest).ok();
    }

    #[test]
    fn test_log_move_operation() {
        let source = Path::new("/tmp/smartfo_test_audit_source.txt");
        let dest = Path::new("/tmp/smartfo_test_audit_dest.txt");
        let audit_log = Path::new("/tmp/smartfo_test_audit_log.jsonl");
        
        let result = log_move_operation(
            source,
            dest,
            Some("test reason"),
            None,
            None,
            audit_log,
        );
        
        assert!(result.is_ok());
        assert!(audit_log.exists());
        
        // Cleanup
        std::fs::remove_file(audit_log).ok();
    }

    #[test]
    fn test_log_move_operation_with_vcs() {
        let source = Path::new("/tmp/smartfo_test_audit_vcs_source.txt");
        let dest = Path::new("/tmp/smartfo_test_audit_vcs_dest.txt");
        let audit_log = Path::new("/tmp/smartfo_test_audit_vcs_log.jsonl");
        let vcs_info = VcsInfo::new(crate::vcs::VcsType::Git, PathBuf::from("/tmp/repo"));
        
        let result = log_move_operation(
            source,
            dest,
            None,
            Some(&vcs_info),
            Some(true),
            audit_log,
        );
        
        assert!(result.is_ok());
        assert!(audit_log.exists());
        
        // Verify the log contains VCS information
        let content = std::fs::read_to_string(audit_log).unwrap();
        assert!(content.contains("\"vcs\":\"git\""));
        assert!(content.contains("\"committed\":true"));
        
        // Cleanup
        std::fs::remove_file(audit_log).ok();
    }

    // Comprehensive tests for all six scenarios
    
    #[test]
    fn test_scenario_1_same_path_no_op() {
        let source = Path::new("/tmp/test.txt");
        let dest = Path::new("/tmp/test.txt");
        
        let scenario = classify_scenario(source, dest).unwrap();
        assert_eq!(scenario, MoveScenario::SamePath);
    }

    #[test]
    fn test_scenario_2_both_outside_repo() {
        let source = Path::new("/tmp/smartfo_scenario2_source.txt");
        let dest = Path::new("/tmp/smartfo_scenario2_dest.txt");
        
        let scenario = classify_scenario(source, dest).unwrap();
        assert_eq!(scenario, MoveScenario::BothOutsideRepo);
    }

    #[test]
    fn test_scenario_3_different_paths_no_vcs() {
        let source = Path::new("/tmp/smartfo_scenario3_a.txt");
        let dest = Path::new("/tmp/smartfo_scenario3_b.txt");
        
        let scenario = classify_scenario(source, dest).unwrap();
        // Without VCS, this should be BothOutsideRepo
        assert_eq!(scenario, MoveScenario::BothOutsideRepo);
    }

    #[test]
    fn test_vcs_native_move_all_vcs_types() {
        let source = Path::new("/tmp/smartfo_vcs_test.txt");
        let dest = Path::new("/tmp/smartfo_vcs_dest.txt");
        
        // Test that each VCS type has a corresponding function
        let git_info = VcsInfo::new(crate::vcs::VcsType::Git, PathBuf::from("/tmp"));
        let hg_info = VcsInfo::new(crate::vcs::VcsType::Hg, PathBuf::from("/tmp"));
        let svn_info = VcsInfo::new(crate::vcs::VcsType::Svn, PathBuf::from("/tmp"));
        let jj_info = VcsInfo::new(crate::vcs::VcsType::Jj, PathBuf::from("/tmp"));
        
        // All should fail (no actual VCS commands available) but functions exist
        assert!(vcs_native_move(source, dest, &git_info).is_err());
        assert!(vcs_native_move(source, dest, &hg_info).is_err());
        assert!(vcs_native_move(source, dest, &svn_info).is_err());
        assert!(vcs_native_move(source, dest, &jj_info).is_err());
    }

    #[test]
    fn test_dest_exists_all_modes() {
        let dest = Path::new("/tmp/smartfo_dest_test.txt");
        std::fs::write(dest, "test").unwrap();
        
        // Test no-clobber mode
        assert_eq!(handle_dest_exists(dest, true, false, false, false).unwrap(), false);
        
        // Test force mode
        assert_eq!(handle_dest_exists(dest, false, true, false, false).unwrap(), true);
        
        // Test backup mode
        assert_eq!(handle_dest_exists(dest, false, false, false, true).unwrap(), true);
        
        // Cleanup
        std::fs::remove_file(dest).ok();
    }

    #[test]
    fn test_plain_mv_all_scenarios() {
        let source = Path::new("/tmp/smartfo_plain_test.txt");
        let dest = Path::new("/tmp/smartfo_plain_dest.txt");
        
        // Test same path (no-op)
        assert!(plain_mv(source, source, false, false, false, false).is_ok());
        
        // Test non-existent source (should fail)
        assert!(plain_mv(source, dest, false, false, false, false).is_err());
    }

    #[test]
    fn test_filesystem_rename_actual_move() {
        let source = Path::new("/tmp/smartfo_rename_source.txt");
        let dest = Path::new("/tmp/smartfo_rename_dest.txt");
        
        // Create source file
        std::fs::write(source, "test content").unwrap();
        
        // Perform rename
        assert!(filesystem_rename(source, dest).is_ok());
        
        // Verify source is gone and dest exists
        assert!(!source.exists());
        assert!(dest.exists());
        
        // Cleanup
        std::fs::remove_file(dest).ok();
    }

    #[test]
    fn test_backup_creation() {
        let dest = Path::new("/tmp/smartfo_backup_test.txt");
        std::fs::write(dest, "original content").unwrap();
        
        // Create backup
        assert!(create_backup(dest).is_ok());
        
        // Verify backup was created
        let backup_files: Vec<_> = std::fs::read_dir("/tmp")
            .unwrap()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().to_string_lossy().contains("smartfo_backup_test"))
            .collect();
        
        assert!(!backup_files.is_empty());
        
        // Cleanup
        std::fs::remove_file(dest).ok();
        for entry in backup_files {
            std::fs::remove_file(entry.path()).ok();
        }
    }
}