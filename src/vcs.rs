//! VCS detection and tracked-file logic
//!
//! This module provides detection for Git, Mercurial, SVN, and Jujutsu repositories.
//! It can discover repo roots and determine whether a given path is tracked by the detected VCS.

use anyhow::{Context, Result};
use std::path::Path;

/// Supported version control systems
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VcsType {
    Git,
    Hg,
    Svn,
    Jj,
}

impl VcsType {
    /// Get the command name for this VCS
    pub fn command(&self) -> &'static str {
        match self {
            VcsType::Git => "git",
            VcsType::Hg => "hg",
            VcsType::Svn => "svn",
            VcsType::Jj => "jj",
        }
    }
}

/// Information about a detected VCS repository
#[derive(Debug, Clone)]
pub struct VcsInfo {
    /// The type of VCS detected
    pub vcs_type: VcsType,
    /// The root directory of the repository
    pub repo_root: std::path::PathBuf,
}

impl VcsInfo {
    /// Create a new VcsInfo
    pub fn new(vcs_type: VcsType, repo_root: std::path::PathBuf) -> Self {
        Self { vcs_type, repo_root }
    }

    /// Get the VCS type
    pub fn vcs_type(&self) -> VcsType {
        self.vcs_type
    }

    /// Get the repository root
    pub fn repo_root(&self) -> &std::path::PathBuf {
        &self.repo_root
    }
}

/// Detect the VCS for a given path
///
/// Returns `None` if no supported VCS is detected.
/// Returns the first VCS found if multiple are present (fallback chain).
#[allow(dead_code)]
pub fn detect_vcs(path: &Path) -> Result<Option<VcsInfo>> {
    // Try each VCS in order of preference
    let vcs_types = [VcsType::Git, VcsType::Hg, VcsType::Svn, VcsType::Jj];

    for vcs_type in vcs_types {
        if let Some(info) = detect_vcs_type(path, vcs_type)? {
            tracing::debug!("Detected VCS: {:?} at {:?}", vcs_type, info.repo_root);
            return Ok(Some(info));
        }
    }

    Ok(None)
}

/// Detect a specific VCS type for a given path
fn detect_vcs_type(path: &Path, vcs_type: VcsType) -> Result<Option<VcsInfo>> {
    let command = vcs_type.command();

    // Check if the VCS command is available
    if !is_command_available(command) {
        tracing::trace!("VCS command '{}' not available, skipping", command);
        return Ok(None);
    }

    // Try to detect the repo root
    let repo_root = match vcs_type {
        VcsType::Git => detect_git_root(path)?,
        VcsType::Hg => detect_hg_root(path)?,
        VcsType::Svn => detect_svn_root(path)?,
        VcsType::Jj => detect_jj_root(path)?,
    };

    if let Some(root) = repo_root {
        Ok(Some(VcsInfo::new(vcs_type, root)))
    } else {
        Ok(None)
    }
}

/// Check if a command is available in PATH
fn is_command_available(command: &str) -> bool {
    std::process::Command::new(command)
        .arg("--version")
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .map(|status| status.success())
        .unwrap_or(false)
}

/// Detect Git repository root
fn detect_git_root(path: &Path) -> Result<Option<std::path::PathBuf>> {
    let output = std::process::Command::new("git")
        .args(["rev-parse", "--show-toplevel"])
        .current_dir(path)
        .output()
        .context("Failed to run git rev-parse")?;

    if output.status.success() {
        let root = std::str::from_utf8(&output.stdout)
            .context("Invalid UTF-8 in git output")?
            .trim();
        Ok(Some(std::path::PathBuf::from(root)))
    } else {
        Ok(None)
    }
}

/// Detect Mercurial repository root
fn detect_hg_root(path: &Path) -> Result<Option<std::path::PathBuf>> {
    let output = std::process::Command::new("hg")
        .args(["root"])
        .current_dir(path)
        .output()
        .context("Failed to run hg root")?;

    if output.status.success() {
        let root = std::str::from_utf8(&output.stdout)
            .context("Invalid UTF-8 in hg output")?
            .trim();
        Ok(Some(std::path::PathBuf::from(root)))
    } else {
        Ok(None)
    }
}

/// Detect SVN repository root
fn detect_svn_root(path: &Path) -> Result<Option<std::path::PathBuf>> {
    let output = std::process::Command::new("svn")
        .args(["info"])
        .current_dir(path)
        .output()
        .context("Failed to run svn info")?;

    if output.status.success() {
        // Parse "Working Copy Root Path" from svn info output
        let stdout = std::str::from_utf8(&output.stdout)
            .context("Invalid UTF-8 in svn output")?;
        
        for line in stdout.lines() {
            if line.starts_with("Working Copy Root Path: ") {
                let root = line.trim_start_matches("Working Copy Root Path: ").trim();
                return Ok(Some(std::path::PathBuf::from(root)));
            }
        }
        Ok(None)
    } else {
        Ok(None)
    }
}

/// Detect Jujutsu repository root
fn detect_jj_root(path: &Path) -> Result<Option<std::path::PathBuf>> {
    let output = std::process::Command::new("jj")
        .args(["root"])
        .current_dir(path)
        .output()
        .context("Failed to run jj root")?;

    if output.status.success() {
        let root = std::str::from_utf8(&output.stdout)
            .context("Invalid UTF-8 in jj output")?
            .trim();
        Ok(Some(std::path::PathBuf::from(root)))
    } else {
        Ok(None)
    }
}

/// Check if a file is tracked by the VCS
///
/// Returns `Ok(true)` if the file is tracked, `Ok(false)` if not tracked,
/// and `Err` if there's an error determining tracked status.
#[allow(dead_code)]
pub fn is_tracked(vcs_info: &VcsInfo, file_path: &Path) -> Result<bool> {
    match vcs_info.vcs_type {
        VcsType::Git => is_git_tracked(vcs_info, file_path),
        VcsType::Hg => is_hg_tracked(vcs_info, file_path),
        VcsType::Svn => is_svn_tracked(vcs_info, file_path),
        VcsType::Jj => is_jj_tracked(vcs_info, file_path),
    }
}

/// Check if a file is tracked by Git
#[allow(dead_code)]
fn is_git_tracked(vcs_info: &VcsInfo, file_path: &Path) -> Result<bool> {
    let output = std::process::Command::new("git")
        .args(["ls-files", "--error-unmatch"])
        .arg(file_path)
        .current_dir(&vcs_info.repo_root)
        .output()
        .context("Failed to run git ls-files")?;

    Ok(output.status.success())
}

/// Check if a file is tracked by Mercurial
fn is_hg_tracked(vcs_info: &VcsInfo, file_path: &Path) -> Result<bool> {
    // Get the relative path from repo root
    let relative_path = file_path
        .strip_prefix(&vcs_info.repo_root)
        .context("File path is not within repo root")?;

    let output = std::process::Command::new("hg")
        .args(["status", "-mu"])
        .current_dir(&vcs_info.repo_root)
        .output()
        .context("Failed to run hg status")?;

    if !output.status.success() {
        return Ok(false);
    }

    let stdout = std::str::from_utf8(&output.stdout)
        .context("Invalid UTF-8 in hg output")?;

    // hg status -mu lists modified and unknown files
    // If the file is NOT in this list, it's tracked and clean
    for line in stdout.lines() {
        if line.ends_with(relative_path.to_str().unwrap_or("")) {
            return Ok(false);
        }
    }

    Ok(true)
}

/// Check if a file is tracked by SVN
#[allow(dead_code)]
fn is_svn_tracked(vcs_info: &VcsInfo, file_path: &Path) -> Result<bool> {
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

    // SVN status returns output for tracked files
    // If the file is not under version control, it won't appear
    Ok(!stdout.trim().is_empty())
}

/// Check if a file is tracked by Jujutsu
#[allow(dead_code)]
fn is_jj_tracked(vcs_info: &VcsInfo, file_path: &Path) -> Result<bool> {
    let output = std::process::Command::new("jj")
        .args(["file", "list"])
        .current_dir(&vcs_info.repo_root)
        .output()
        .context("Failed to run jj file list")?;

    if !output.status.success() {
        return Ok(false);
    }

    let stdout = std::str::from_utf8(&output.stdout)
        .context("Invalid UTF-8 in jj output")?;

    // Get the relative path from repo root
    let relative_path = file_path
        .strip_prefix(&vcs_info.repo_root)
        .context("File path is not within repo root")?
        .to_str()
        .context("Invalid UTF-8 in file path")?;

    // Check if the file is in the list
    Ok(stdout.lines().any(|line| line.trim() == relative_path))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vcs_type_command() {
        assert_eq!(VcsType::Git.command(), "git");
        assert_eq!(VcsType::Hg.command(), "hg");
        assert_eq!(VcsType::Svn.command(), "svn");
        assert_eq!(VcsType::Jj.command(), "jj");
    }

    #[test]
    fn test_vcs_info_new() {
        let info = VcsInfo::new(VcsType::Git, std::path::PathBuf::from("/test"));
        assert_eq!(info.vcs_type, VcsType::Git);
        assert_eq!(info.repo_root, std::path::PathBuf::from("/test"));
    }
}
