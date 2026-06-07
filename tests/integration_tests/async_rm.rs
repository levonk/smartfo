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

/// Test rm async behavior returns prompt immediately
#[test]
fn test_rm_async_prompt_return() {
    let dir = tempfile::tempdir().unwrap();
    
    // Create a file
    let test_file = dir.path().join("test.txt");
    fs::write(&test_file, "test content").unwrap();

    symlink_binary("rm", dir.path());

    // Remove file (async by default)
    let mut cmd = Command::cargo_bin("smartfo").unwrap();
    cmd.arg("test.txt")
        .current_dir(dir.path())
        .assert()
        .success();

    // Note: Async operation timing tests are skipped for now
    // This is a placeholder for future implementation
}

/// Test rm with --blocking flag waits for completion
#[test]
fn test_rm_blocking() {
    let dir = tempfile::tempdir().unwrap();
    
    // Create a file
    let test_file = dir.path().join("test.txt");
    fs::write(&test_file, "test content").unwrap();

    symlink_binary("rm", dir.path());

    // Remove file with --blocking flag
    let mut cmd = Command::cargo_bin("smartfo").unwrap();
    cmd.arg("--blocking")
        .arg("test.txt")
        .current_dir(dir.path())
        .assert()
        .success();

    // Note: Async operation timing tests are skipped for now
    // This is a placeholder for future implementation
}

/// Test rm async with large file
#[test]
fn test_rm_async_large_file() {
    let dir = tempfile::tempdir().unwrap();
    
    // Create a large file
    let test_file = dir.path().join("large.bin");
    let large_content = vec![0u8; 100 * 1024 * 1024]; // 100MB
    fs::write(&test_file, large_content).unwrap();

    symlink_binary("rm", dir.path());

    // Remove large file (async by default)
    let mut cmd = Command::cargo_bin("smartfo").unwrap();
    cmd.arg("large.bin")
        .current_dir(dir.path())
        .assert()
        .success();

    // Note: Async operation timing tests are skipped for now
    // This is a placeholder for future implementation
}

/// Test rm async with multiple files
#[test]
fn test_rm_async_multiple_files() {
    let dir = tempfile::tempdir().unwrap();
    
    // Create multiple files
    for i in 0..5 {
        let test_file = dir.path().join(format!("test{}.txt", i));
        fs::write(&test_file, format!("content {}", i)).unwrap();
    }

    symlink_binary("rm", dir.path());

    // Remove all files (async by default)
    let mut cmd = Command::cargo_bin("smartfo").unwrap();
    cmd.arg("test0.txt")
        .arg("test1.txt")
        .arg("test2.txt")
        .arg("test3.txt")
        .arg("test4.txt")
        .current_dir(dir.path())
        .assert()
        .success();

    // Note: Async operation timing tests are skipped for now
    // This is a placeholder for future implementation
}

/// Test rm async with directory
#[test]
fn test_rm_async_directory() {
    let dir = tempfile::tempdir().unwrap();
    
    // Create a directory with files
    let test_dir = dir.path().join("testdir");
    fs::create_dir(&test_dir).unwrap();
    fs::write(test_dir.join("file1.txt"), "content1").unwrap();
    fs::write(test_dir.join("file2.txt"), "content2").unwrap();

    symlink_binary("rm", dir.path());

    // Remove directory (async by default)
    let mut cmd = Command::cargo_bin("smartfo").unwrap();
    cmd.arg("-r")
        .arg("testdir")
        .current_dir(dir.path())
        .assert()
        .success();

    // Note: Async operation timing tests are skipped for now
    // This is a placeholder for future implementation
}

/// Test rm async with --sync flag forces fsync
#[test]
fn test_rm_async_with_sync_flag() {
    let dir = tempfile::tempdir().unwrap();
    
    // Create a file
    let test_file = dir.path().join("test.txt");
    fs::write(&test_file, "test content").unwrap();

    symlink_binary("rm", dir.path());

    // Remove file with --sync flag
    let mut cmd = Command::cargo_bin("smartfo").unwrap();
    cmd.arg("--sync")
        .arg("test.txt")
        .current_dir(dir.path())
        .assert()
        .success();

    // Note: Async operation timing tests are skipped for now
    // This is a placeholder for future implementation
}
