use assert_cmd::Command;
use assert_cmd::assert::OutputAssertExt;
use predicates::prelude::*;
use std::fs;
use std::path::Path;
use super::fixtures::{create_git_repo, create_tracked_file, create_untracked_file};

/// Create a symlink to the smartfo binary with the given name.
fn symlink_binary(name: &str, tmp_dir: &Path) {
    let bin = Command::cargo_bin("smartfo").unwrap();
    let dest = tmp_dir.join(name);
    #[cfg(unix)]
    std::os::unix::fs::symlink(bin.get_program(), &dest).unwrap();
    #[cfg(windows)]
    std::os::windows::fs::symlink_file(bin.get_program(), &dest).unwrap();
}

/// Scenario 1: Tracked source → same repo (VCS-native move)
#[test]
fn test_mv_tracked_to_tracked_same_repo() {
    let repo = create_git_repo();
    let src_path = repo.path().join("file1.txt");
    let dest_path = repo.path().join("file2.txt");

    create_tracked_file(repo.path(), "file1.txt", "content");

    symlink_binary("mv", repo.path());

    let mut cmd = std::process::Command::new(repo.path().join("mv"));
    cmd.arg("file1.txt")
        .arg("file2.txt")
        .current_dir(repo.path())
        .assert()
        .success();

    // Verify source is gone
    assert!(!src_path.exists());
    // Verify dest exists
    assert!(dest_path.exists());
    // Verify file is tracked in git
    let output = Command::new("git")
        .args(["status", "--porcelain"])
        .current_dir(repo.path())
        .output()
        .unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout);
    // Should show renamed file
    assert!(stdout.contains("file2.txt"));
}

/// Scenario 2: Tracked source → outside repo (should refuse without --force-outside-vcs)
#[test]
fn test_mv_tracked_to_outside_repo_refuses() {
    let repo = create_git_repo();
    let outside_dir = tempfile::tempdir().unwrap();

    create_tracked_file(repo.path(), "file1.txt", "content");

    symlink_binary("mv", repo.path());

    let mut cmd = std::process::Command::new(repo.path().join("mv"));
    cmd.arg("file1.txt")
        .arg(outside_dir.path().join("file1.txt"))
        .current_dir(repo.path())
        .assert()
        .failure()
        .stderr(predicate::str::contains("tracked file"));

    // Verify source still exists
    assert!(repo.path().join("file1.txt").exists());
}

/// Scenario 2: Tracked source → outside repo (with --force-outside-vcs)
#[test]
fn test_mv_tracked_to_outside_repo_with_force() {
    let repo = create_git_repo();
    let outside_dir = tempfile::tempdir().unwrap();

    create_tracked_file(repo.path(), "file1.txt", "content");

    symlink_binary("mv", repo.path());

    let mut cmd = std::process::Command::new(repo.path().join("mv"));
    cmd.arg("--force-outside-vcs")
        .arg("file1.txt")
        .arg(outside_dir.path().join("file1.txt"))
        .current_dir(repo.path())
        .assert()
        .success();

    // Verify source is gone
    assert!(!repo.path().join("file1.txt").exists());
    // Verify dest exists
    assert!(outside_dir.path().join("file1.txt").exists());
}

/// Scenario 3: Outside repo → inside repo (Filesystem move)
#[test]
fn test_mv_outside_to_inside_repo() {
    let repo = create_git_repo();
    let outside_dir = tempfile::tempdir().unwrap();

    fs::write(outside_dir.path().join("file1.txt"), "content").unwrap();

    symlink_binary("mv", outside_dir.path());

    let mut cmd = std::process::Command::new(outside_dir.path().join("mv"));
    cmd.arg("file1.txt")
        .arg(repo.path().join("file1.txt"))
        .current_dir(outside_dir.path())
        .assert()
        .success();

    // Verify source is gone
    assert!(!outside_dir.path().join("file1.txt").exists());
    // Verify dest exists
    assert!(repo.path().join("file1.txt").exists());
}

/// Scenario 4: Both outside any repo (Pure filesystem rename)
#[test]
fn test_mv_both_outside_repo() {
    let dir1 = tempfile::tempdir().unwrap();
    let dir2 = tempfile::tempdir().unwrap();

    fs::write(dir1.path().join("file1.txt"), "content").unwrap();

    symlink_binary("mv", dir1.path());

    let mut cmd = std::process::Command::new(dir1.path().join("mv"));
    cmd.arg("file1.txt")
        .arg(dir2.path().join("file1.txt"))
        .current_dir(dir1.path())
        .assert()
        .success();

    // Verify source is gone
    assert!(!dir1.path().join("file1.txt").exists());
    // Verify dest exists
    assert!(dir2.path().join("file1.txt").exists());
}

/// Scenario 5: Neither tracked in repo (Pure filesystem rename)
#[test]
fn test_mv_untracked_in_repo() {
    let repo = create_git_repo();

    create_untracked_file(repo.path(), "file1.txt", "content");

    symlink_binary("mv", repo.path());

    let mut cmd = std::process::Command::new(repo.path().join("mv"));
    cmd.arg("file1.txt")
        .arg("file2.txt")
        .current_dir(repo.path())
        .assert()
        .success();

    // Verify source is gone
    assert!(!repo.path().join("file1.txt").exists());
    // Verify dest exists
    assert!(repo.path().join("file2.txt").exists());
}

/// Scenario 6: src == dest (No-op, exit 0)
#[test]
fn test_mv_same_file_noop() {
    let repo = create_git_repo();

    create_tracked_file(repo.path(), "file1.txt", "content");

    symlink_binary("mv", repo.path());

    let mut cmd = std::process::Command::new(repo.path().join("mv"));
    cmd.arg("file1.txt")
        .arg("file1.txt")
        .current_dir(repo.path())
        .assert()
        .success();

    // Verify file still exists
    assert!(repo.path().join("file1.txt").exists());
}

/// Test moving directory with tracked files
#[test]
fn test_mv_directory_tracked() {
    let repo = create_git_repo();
    let dir_path = repo.path().join("dir1");
    let dest_dir = repo.path().join("dir2");

    fs::create_dir(&dir_path).unwrap();
    create_tracked_file(&dir_path, "file1.txt", "content");

    symlink_binary("mv", repo.path());

    let mut cmd = std::process::Command::new(repo.path().join("mv"));
    cmd.arg("dir1")
        .arg("dir2")
        .current_dir(repo.path())
        .assert()
        .success();

    // Verify source dir is gone
    assert!(!dir_path.exists());
    // Verify dest dir exists with file
    assert!(dest_dir.exists());
    assert!(dest_dir.join("file1.txt").exists());
}

/// Test moving multiple files
#[test]
fn test_mv_multiple_files() {
    let repo = create_git_repo();
    let dest_dir = repo.path().join("dest");

    fs::create_dir(&dest_dir).unwrap();
    create_tracked_file(repo.path(), "file1.txt", "content");
    create_tracked_file(repo.path(), "file2.txt", "content");

    symlink_binary("mv", repo.path());

    let mut cmd = std::process::Command::new(repo.path().join("mv"));
    cmd.arg("file1.txt")
        .arg("file2.txt")
        .arg("dest")
        .current_dir(repo.path())
        .assert()
        .success();

    // Verify sources are gone
    assert!(!repo.path().join("file1.txt").exists());
    assert!(!repo.path().join("file2.txt").exists());
    // Verify dest files exist
    assert!(dest_dir.join("file1.txt").exists());
    assert!(dest_dir.join("file2.txt").exists());
}
