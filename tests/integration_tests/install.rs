use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use std::path::Path;
use tempfile::TempDir;

/// Test --install creates symlinks in XDG_BIN_HOME
#[test]
fn test_install_creates_symlinks_xdg_bin_home() {
    let dir = tempfile::tempdir().unwrap();
    
    // Set XDG_BIN_HOME to temp directory
    std::env::set_var("XDG_BIN_HOME", dir.path());

    let mut cmd = Command::cargo_bin("smartfo").unwrap();
    cmd.arg("--install")
        .assert()
        .success();

    // Verify symlinks were created
    assert!(dir.path().join("mv").exists());
    assert!(dir.path().join("rm").exists());
    assert!(dir.path().join("smv").exists());
    assert!(dir.path().join("srm").exists());

    // Cleanup
    std::env::remove_var("XDG_BIN_HOME");
}

/// Test --install creates symlinks in ~/.local/bin if XDG_BIN_HOME not set
#[test]
fn test_install_creates_symlinks_local_bin() {
    let dir = tempfile::tempdir().unwrap();
    
    // Set HOME to temp directory
    std::env::set_var("HOME", dir.path());
    // Ensure XDG_BIN_HOME is not set
    std::env::remove_var("XDG_BIN_HOME");

    let mut cmd = Command::cargo_bin("smartfo").unwrap();
    cmd.arg("--install")
        .assert()
        .success();

    // Verify symlinks were created in ~/.local/bin
    let local_bin = dir.path().join(".local").join("bin");
    assert!(local_bin.join("mv").exists());
    assert!(local_bin.join("rm").exists());
    assert!(local_bin.join("smv").exists());
    assert!(local_bin.join("srm").exists());

    // Cleanup
    std::env::remove_var("HOME");
}

/// Test --install refuses to overwrite existing files without --force
#[test]
fn test_install_refuses_overwrite_without_force() {
    let dir = tempfile::tempdir().unwrap();
    
    // Set XDG_BIN_HOME to temp directory
    std::env::set_var("XDG_BIN_HOME", dir.path());

    // Create existing files
    fs::write(dir.path().join("mv"), "existing mv").unwrap();
    fs::write(dir.path().join("rm"), "existing rm").unwrap();

    let mut cmd = Command::cargo_bin("smartfo").unwrap();
    cmd.arg("--install")
        .assert()
        .failure()
        .stderr(predicate::str::contains("already exists"));

    // Verify existing files were not overwritten
    assert_eq!(fs::read_to_string(dir.path().join("mv")).unwrap(), "existing mv");
    assert_eq!(fs::read_to_string(dir.path().join("rm")).unwrap(), "existing rm");

    // Cleanup
    std::env::remove_var("XDG_BIN_HOME");
}

/// Test --install with --force overwrites existing files
#[test]
fn test_install_force_overwrites() {
    let dir = tempfile::tempdir().unwrap();
    
    // Set XDG_BIN_HOME to temp directory
    std::env::set_var("XDG_BIN_HOME", dir.path());

    // Create existing files
    fs::write(dir.path().join("mv"), "existing mv").unwrap();
    fs::write(dir.path().join("rm"), "existing rm").unwrap();

    let mut cmd = Command::cargo_bin("smartfo").unwrap();
    cmd.arg("--install")
        .arg("--force")
        .assert()
        .success();

    // Verify files were overwritten (now symlinks)
    assert!(dir.path().join("mv").exists());
    assert!(dir.path().join("rm").exists());

    // Cleanup
    std::env::remove_var("XDG_BIN_HOME");
}

/// Test --install creates ~/.local/bin if it doesn't exist
#[test]
fn test_install_creates_local_bin_directory() {
    let dir = tempfile::tempdir().unwrap();
    
    // Set HOME to temp directory
    std::env::set_var("HOME", dir.path());
    // Ensure XDG_BIN_HOME is not set
    std::env::remove_var("XDG_BIN_HOME");

    // Verify .local/bin doesn't exist
    let local_bin = dir.path().join(".local").join("bin");
    assert!(!local_bin.exists());

    let mut cmd = Command::cargo_bin("smartfo").unwrap();
    cmd.arg("--install")
        .assert()
        .success();

    // Verify .local/bin was created
    assert!(local_bin.exists());
    assert!(local_bin.join("mv").exists());
    assert!(local_bin.join("rm").exists());

    // Cleanup
    std::env::remove_var("HOME");
}

/// Test --install with --hooks client installs pre-commit hook
#[test]
fn test_install_hooks_client() {
    let dir = tempfile::tempdir().unwrap();
    
    // Set XDG_BIN_HOME to temp directory
    std::env::set_var("XDG_BIN_HOME", dir.path());

    // Create a git repo
    let repo_dir = tempfile::tempdir().unwrap();
    let _ = std::process::Command::new("git")
        .arg("init")
        .current_dir(repo_dir.path())
        .output();

    let mut cmd = Command::cargo_bin("smartfo").unwrap();
    cmd.arg("--install")
        .arg("--hooks")
        .arg("client")
        .current_dir(repo_dir.path())
        .assert()
        .success();

    // Verify pre-commit hook was installed
    let hooks_dir = repo_dir.path().join(".git").join("hooks");
    let pre_commit = hooks_dir.join("pre-commit");
    assert!(pre_commit.exists());

    // Cleanup
    std::env::remove_var("XDG_BIN_HOME");
}

/// Test --install with --hooks server installs pre-receive hook
#[test]
fn test_install_hooks_server() {
    let dir = tempfile::tempdir().unwrap();
    
    // Set XDG_BIN_HOME to temp directory
    std::env::set_var("XDG_BIN_HOME", dir.path());

    // Create a git repo
    let repo_dir = tempfile::tempdir().unwrap();
    let _ = std::process::Command::new("git")
        .arg("init")
        .current_dir(repo_dir.path())
        .output();

    let mut cmd = Command::cargo_bin("smartfo").unwrap();
    cmd.arg("--install")
        .arg("--hooks")
        .arg("server")
        .current_dir(repo_dir.path())
        .assert()
        .success();

    // Verify pre-receive hook was installed
    let hooks_dir = repo_dir.path().join(".git").join("hooks");
    let pre_receive = hooks_dir.join("pre-receive");
    assert!(pre_receive.exists());

    // Cleanup
    std::env::remove_var("XDG_BIN_HOME");
}

/// Test --install with --no-hooks skips hook installation
#[test]
fn test_install_no_hooks() {
    let dir = tempfile::tempdir().unwrap();
    
    // Set XDG_BIN_HOME to temp directory
    std::env::set_var("XDG_BIN_HOME", dir.path());

    // Create a git repo
    let repo_dir = tempfile::tempdir().unwrap();
    let _ = std::process::Command::new("git")
        .arg("init")
        .current_dir(repo_dir.path())
        .output();

    let mut cmd = Command::cargo_bin("smartfo").unwrap();
    cmd.arg("--install")
        .arg("--no-hooks")
        .current_dir(repo_dir.path())
        .assert()
        .success();

    // Verify hooks were not installed
    let hooks_dir = repo_dir.path().join(".git").join("hooks");
    let pre_commit = hooks_dir.join("pre-commit");
    let pre_receive = hooks_dir.join("pre-receive");
    assert!(!pre_commit.exists());
    assert!(!pre_receive.exists());

    // Cleanup
    std::env::remove_var("XDG_BIN_HOME");
}

/// Test --install with --hooks client,server installs both hooks
#[test]
fn test_install_hooks_both() {
    let dir = tempfile::tempdir().unwrap();
    
    // Set XDG_BIN_HOME to temp directory
    std::env::set_var("XDG_BIN_HOME", dir.path());

    // Create a git repo
    let repo_dir = tempfile::tempdir().unwrap();
    let _ = std::process::Command::new("git")
        .arg("init")
        .current_dir(repo_dir.path())
        .output();

    let mut cmd = Command::cargo_bin("smartfo").unwrap();
    cmd.arg("--install")
        .arg("--hooks")
        .arg("client,server")
        .current_dir(repo_dir.path())
        .assert()
        .success();

    // Verify both hooks were installed
    let hooks_dir = repo_dir.path().join(".git").join("hooks");
    let pre_commit = hooks_dir.join("pre-commit");
    let pre_receive = hooks_dir.join("pre-receive");
    assert!(pre_commit.exists());
    assert!(pre_receive.exists());

    // Cleanup
    std::env::remove_var("XDG_BIN_HOME");
}
