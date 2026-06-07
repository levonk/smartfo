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

/// Test large file async mv returns prompt immediately
#[test]
fn test_large_file_async_mv_prompt_return() {
    let dir = tempfile::tempdir().unwrap();
    
    // Create a large file (>100MB to trigger async)
    let source_file = dir.path().join("large.bin");
    let large_content = vec![0u8; 100 * 1024 * 1024]; // 100MB
    fs::write(&source_file, large_content).unwrap();

    symlink_binary("mv", dir.path());

    // Move file with --async flag
    let dest_file = dir.path().join("large_moved.bin");
    let mut cmd = Command::cargo_bin("smartfo").unwrap();
    cmd.arg("--async")
        .arg("large.bin")
        .arg("large_moved.bin")
        .current_dir(dir.path())
        .assert()
        .success();

    // Note: Async operation timing tests are skipped for now
    // This is a placeholder for future implementation
}

/// Test large file async mv with --blocking flag waits for completion
#[test]
fn test_large_file_async_mv_blocking() {
    let dir = tempfile::tempdir().unwrap();
    
    // Create a large file (>100MB to trigger async)
    let source_file = dir.path().join("large.bin");
    let large_content = vec![0u8; 100 * 1024 * 1024]; // 100MB
    fs::write(&source_file, large_content).unwrap();

    symlink_binary("mv", dir.path());

    // Move file with --blocking flag
    let dest_file = dir.path().join("large_moved.bin");
    let mut cmd = Command::cargo_bin("smartfo").unwrap();
    cmd.arg("--blocking")
        .arg("large.bin")
        .arg("large_moved.bin")
        .current_dir(dir.path())
        .assert()
        .success();

    // Note: Async operation timing tests are skipped for now
    // This is a placeholder for future implementation
}

/// Test small file mv is synchronous by default
#[test]
fn test_small_file_mv_sync_default() {
    let dir = tempfile::tempdir().unwrap();
    
    // Create a small file (<100MB)
    let source_file = dir.path().join("small.txt");
    fs::write(&source_file, "small content").unwrap();

    symlink_binary("mv", dir.path());

    // Move file without --async flag
    let dest_file = dir.path().join("small_moved.txt");
    let mut cmd = Command::cargo_bin("smartfo").unwrap();
    cmd.arg("small.txt")
        .arg("small_moved.txt")
        .current_dir(dir.path())
        .assert()
        .success();

    // Verify source is gone
    assert!(!source_file.exists());
    // Verify dest exists
    assert!(dest_file.exists());
}

/// Test async mv with multiple large files
#[test]
fn test_async_mv_multiple_large_files() {
    let dir = tempfile::tempdir().unwrap();
    
    // Create multiple large files
    for i in 0..3 {
        let source_file = dir.path().join(format!("large{}.bin", i));
        let large_content = vec![0u8; 50 * 1024 * 1024]; // 50MB each
        fs::write(&source_file, large_content).unwrap();
    }

    symlink_binary("mv", dir.path());

    let dest_dir = dir.path().join("dest");
    fs::create_dir(&dest_dir).unwrap();

    // Move all files with --async flag
    let mut cmd = Command::cargo_bin("smartfo").unwrap();
    cmd.arg("--async")
        .arg("large0.bin")
        .arg("large1.bin")
        .arg("large2.bin")
        .arg("dest")
        .current_dir(dir.path())
        .assert()
        .success();

    // Note: Async operation timing tests are skipped for now
    // This is a placeholder for future implementation
}

/// Test async mv with --sync flag forces fsync
#[test]
fn test_async_mv_with_sync_flag() {
    let dir = tempfile::tempdir().unwrap();
    
    // Create a large file
    let source_file = dir.path().join("large.bin");
    let large_content = vec![0u8; 100 * 1024 * 1024]; // 100MB
    fs::write(&source_file, large_content).unwrap();

    symlink_binary("mv", dir.path());

    // Move file with --async and --sync flags
    let dest_file = dir.path().join("large_moved.bin");
    let mut cmd = Command::cargo_bin("smartfo").unwrap();
    cmd.arg("--async")
        .arg("--sync")
        .arg("large.bin")
        .arg("large_moved.bin")
        .current_dir(dir.path())
        .assert()
        .success();

    // Note: Async operation timing tests are skipped for now
    // This is a placeholder for future implementation
}
