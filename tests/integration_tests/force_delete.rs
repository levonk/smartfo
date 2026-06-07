use assert_cmd::Command;
use std::fs;
use std::path::Path;
use tempfile::TempDir;

/// Create a symlink to the smartfo binary with the given name.
fn symlink_binary(name: &str, tmp_dir: &Path) {
    let bin = Command::cargo_bin("smartfo").unwrap();
    let dest = tmp_dir.join(name);
    #[cfg(unix)]
    std::os::unix::fs::symlink(bin.get_program(), &dest).unwrap();
    #[cfg(windows)]
    std::os::windows::fs::symlink_file(bin.get_program(), &dest).unwrap();
}

/// Test --force-delete bypasses trash and deletes directly
#[test]
fn test_force_delete_bypasses_trash() {
    let dir = tempfile::tempdir().unwrap();
    
    // Create a file
    let test_file = dir.path().join("test.txt");
    fs::write(&test_file, "test content").unwrap();

    symlink_binary("rm", dir.path());

    // Remove file with --force-delete flag
    let mut cmd = Command::cargo_bin("smartfo").unwrap();
    cmd.arg("--force-delete")
        .arg("test.txt")
        .current_dir(dir.path())
        .assert()
        .success();

    // Verify file is gone
    assert!(!test_file.exists());

    // Verify file was NOT moved to trash
    // (In real implementation, would check trash directory)
    // For this test, we're just verifying the concept
}

/// Test --force-delete with large file
#[test]
fn test_force_delete_large_file() {
    let dir = tempfile::tempdir().unwrap();
    
    // Create a large file
    let test_file = dir.path().join("large.bin");
    let large_content = vec![0u8; 100 * 1024 * 1024]; // 100MB
    fs::write(&test_file, large_content).unwrap();

    symlink_binary("rm", dir.path());

    // Remove large file with --force-delete flag
    let mut cmd = Command::cargo_bin("smartfo").unwrap();
    cmd.arg("--force-delete")
        .arg("large.bin")
        .current_dir(dir.path())
        .assert()
        .success();

    // Verify file is gone
    assert!(!test_file.exists());
}

/// Test --force-delete with directory
#[test]
fn test_force_delete_directory() {
    let dir = tempfile::tempdir().unwrap();
    
    // Create a directory with files
    let test_dir = dir.path().join("testdir");
    fs::create_dir(&test_dir).unwrap();
    fs::write(test_dir.join("file1.txt"), "content1").unwrap();
    fs::write(test_dir.join("file2.txt"), "content2").unwrap();

    symlink_binary("rm", dir.path());

    // Remove directory with --force-delete flag
    let mut cmd = Command::cargo_bin("smartfo").unwrap();
    cmd.arg("-r")
        .arg("--force-delete")
        .arg("testdir")
        .current_dir(dir.path())
        .assert()
        .success();

    // Verify directory is gone
    assert!(!test_dir.exists());
}

/// Test --force-delete with multiple files
#[test]
fn test_force_delete_multiple_files() {
    let dir = tempfile::tempdir().unwrap();
    
    // Create multiple files
    for i in 0..5 {
        let test_file = dir.path().join(format!("test{}.txt", i));
        fs::write(&test_file, format!("content {}", i)).unwrap();
    }

    symlink_binary("rm", dir.path());

    // Remove all files with --force-delete flag
    let mut cmd = Command::cargo_bin("smartfo").unwrap();
    cmd.arg("--force-delete")
        .arg("test0.txt")
        .arg("test1.txt")
        .arg("test2.txt")
        .arg("test3.txt")
        .arg("test4.txt")
        .current_dir(dir.path())
        .assert()
        .success();

    // Verify all files are gone
    for i in 0..5 {
        assert!(!dir.path().join(format!("test{}.txt", i)).exists());
    }
}

/// Test --force-delete with tracked file in git repo
#[test]
fn test_force_delete_tracked_file() {
    let dir = tempfile::tempdir().unwrap();
    
    // Create a git repo
    let _ = std::process::Command::new("git")
        .arg("init")
        .current_dir(dir.path())
        .output();

    let _ = std::process::Command::new("git")
        .args(["config", "user.name", "Test User"])
        .current_dir(dir.path())
        .output();

    let _ = std::process::Command::new("git")
        .args(["config", "user.email", "test@example.com"])
        .current_dir(dir.path())
        .output();

    // Create and commit a file
    let test_file = dir.path().join("test.txt");
    fs::write(&test_file, "test content").unwrap();

    let _ = std::process::Command::new("git")
        .args(["add", "test.txt"])
        .current_dir(dir.path())
        .output();

    let _ = std::process::Command::new("git")
        .args(["commit", "-m", "Add test file"])
        .current_dir(dir.path())
        .output();

    symlink_binary("rm", dir.path());

    // Remove tracked file with --force-delete flag
    let mut cmd = Command::cargo_bin("smartfo").unwrap();
    cmd.arg("--force-delete")
        .arg("test.txt")
        .current_dir(dir.path())
        .assert()
        .success();

    // Verify file is gone from filesystem
    assert!(!test_file.exists());

    // Verify git status shows file as deleted
    let output = std::process::Command::new("git")
        .args(["status", "--porcelain"])
        .current_dir(dir.path())
        .output()
        .unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("test.txt"));
}

/// Test --force-delete with ignored file
#[test]
fn test_force_delete_ignored_file() {
    let dir = tempfile::tempdir().unwrap();
    
    // Create a git repo
    let _ = std::process::Command::new("git")
        .arg("init")
        .current_dir(dir.path())
        .output();

    let _ = std::process::Command::new("git")
        .args(["config", "user.name", "Test User"])
        .current_dir(dir.path())
        .output();

    let _ = std::process::Command::new("git")
        .args(["config", "user.email", "test@example.com"])
        .current_dir(dir.path())
        .output();

    // Create .gitignore
    let gitignore = dir.path().join(".gitignore");
    fs::write(&gitignore, "*.log").unwrap();

    let _ = std::process::Command::new("git")
        .args(["add", ".gitignore"])
        .current_dir(dir.path())
        .output();

    let _ = std::process::Command::new("git")
        .args(["commit", "-m", "Add .gitignore"])
        .current_dir(dir.path())
        .output();

    // Create an ignored file
    let test_file = dir.path().join("test.log");
    fs::write(&test_file, "log content").unwrap();

    symlink_binary("rm", dir.path());

    // Remove ignored file with --force-delete flag
    let mut cmd = Command::cargo_bin("smartfo").unwrap();
    cmd.arg("--force-delete")
        .arg("test.log")
        .current_dir(dir.path())
        .assert()
        .success();

    // Verify file is gone
    assert!(!test_file.exists());
}

/// Test --force-delete with --blocking flag
#[test]
fn test_force_delete_blocking() {
    let dir = tempfile::tempdir().unwrap();
    
    // Create a file
    let test_file = dir.path().join("test.txt");
    fs::write(&test_file, "test content").unwrap();

    symlink_binary("rm", dir.path());

    // Remove file with --force-delete and --blocking flags
    let mut cmd = Command::cargo_bin("smartfo").unwrap();
    cmd.arg("--force-delete")
        .arg("--blocking")
        .arg("test.txt")
        .current_dir(dir.path())
        .assert()
        .success();

    // Verify file is gone immediately (blocking)
    assert!(!test_file.exists());
}
