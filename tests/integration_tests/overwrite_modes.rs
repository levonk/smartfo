use assert_cmd::Command;
use assert_cmd::assert::OutputAssertExt;
use predicates::prelude::*;
use std::fs;
use std::path::Path;
use std::process::Command as SysCommand;
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

/// Test -n (no-clobber) flag: refuse to overwrite existing file
#[test]
fn test_mv_no_clobber_flag() {
    let dir = tempfile::tempdir().unwrap();
    
    // Create source and destination files
    let source_file = dir.path().join("source.txt");
    let dest_file = dir.path().join("dest.txt");
    fs::write(&source_file, "source content").unwrap();
    fs::write(&dest_file, "dest content").unwrap();

    symlink_binary("mv", dir.path());

    // Try to move with -n flag
    let mut cmd = Command::cargo_bin("smartfo").unwrap();
    cmd.arg("-n")
        .arg("source.txt")
        .arg("dest.txt")
        .current_dir(dir.path())
        .assert()
        .success();

    // Verify source still exists (not overwritten)
    assert!(source_file.exists());
    // Verify dest unchanged
    assert_eq!(fs::read_to_string(&dest_file).unwrap(), "dest content");
}

/// Test -f (force) flag: overwrite existing file
#[test]
fn test_mv_force_flag() {
    let dir = tempfile::tempdir().unwrap();
    
    // Create source and destination files
    let source_file = dir.path().join("source.txt");
    let dest_file = dir.path().join("dest.txt");
    fs::write(&source_file, "source content").unwrap();
    fs::write(&dest_file, "dest content").unwrap();

    symlink_binary("mv", dir.path());

    // Move with -f flag
    let mut cmd = Command::cargo_bin("smartfo").unwrap();
    cmd.arg("-f")
        .arg("source.txt")
        .arg("dest.txt")
        .current_dir(dir.path())
        .assert()
        .success();

    // Verify source is gone
    assert!(!source_file.exists());
    // Verify dest was overwritten
    assert_eq!(fs::read_to_string(&dest_file).unwrap(), "source content");
}

/// Test -i (interactive) flag: prompt before overwrite
#[test]
fn test_mv_interactive_flag() {
    let dir = tempfile::tempdir().unwrap();
    
    // Create source and destination files
    let source_file = dir.path().join("source.txt");
    let dest_file = dir.path().join("dest.txt");
    fs::write(&source_file, "source content").unwrap();
    fs::write(&dest_file, "dest content").unwrap();

    symlink_binary("mv", dir.path());

    // Try to move with -i flag (should prompt)
    // Note: Interactive mode tests are skipped for now as they require stdin handling
    // This is a placeholder for future implementation
}

/// Test -i (interactive) flag with "yes" response
#[test]
fn test_mv_interactive_flag_yes() {
    let dir = tempfile::tempdir().unwrap();
    
    // Create source and destination files
    let source_file = dir.path().join("source.txt");
    let dest_file = dir.path().join("dest.txt");
    fs::write(&source_file, "source content").unwrap();
    fs::write(&dest_file, "dest content").unwrap();

    symlink_binary("mv", dir.path());

    // Note: Interactive mode tests are skipped for now as they require stdin handling
    // This is a placeholder for future implementation
}

/// Test --backup flag: create backup before overwrite
#[test]
fn test_mv_backup_flag() {
    let dir = tempfile::tempdir().unwrap();
    
    // Create source and destination files
    let source_file = dir.path().join("source.txt");
    let dest_file = dir.path().join("dest.txt");
    fs::write(&source_file, "source content").unwrap();
    fs::write(&dest_file, "dest content").unwrap();

    symlink_binary("mv", dir.path());

    // Move with --backup flag
    let mut cmd = Command::cargo_bin("smartfo").unwrap();
    cmd.arg("--backup")
        .arg("source.txt")
        .arg("dest.txt")
        .current_dir(dir.path())
        .assert()
        .success();

    // Verify source is gone
    assert!(!source_file.exists());
    // Verify dest was overwritten
    assert_eq!(fs::read_to_string(&dest_file).unwrap(), "source content");
    // Verify backup file exists
    let backup_file = dir.path().join("dest.txt~");
    assert!(backup_file.exists());
    assert_eq!(fs::read_to_string(&backup_file).unwrap(), "dest content");
}

/// Test --backup=numbered flag: create numbered backup
#[test]
fn test_mv_backup_numbered_flag() {
    let dir = tempfile::tempdir().unwrap();
    
    // Create source and destination files
    let source_file = dir.path().join("source.txt");
    let dest_file = dir.path().join("dest.txt");
    fs::write(&source_file, "source content").unwrap();
    fs::write(&dest_file, "dest content").unwrap();

    symlink_binary("mv", dir.path());

    // Move with --backup=numbered flag
    let mut cmd = Command::cargo_bin("smartfo").unwrap();
    cmd.arg("--backup=numbered")
        .arg("source.txt")
        .arg("dest.txt")
        .current_dir(dir.path())
        .assert()
        .success();

    // Verify source is gone
    assert!(!source_file.exists());
    // Verify dest was overwritten
    assert_eq!(fs::read_to_string(&dest_file).unwrap(), "source content");
    // Verify numbered backup file exists
    let backup_file = dir.path().join("dest.txt.~1~");
    assert!(backup_file.exists());
    assert_eq!(fs::read_to_string(&backup_file).unwrap(), "dest content");
}

/// Test --backup=existing flag: only backup if backup exists
#[test]
fn test_mv_backup_existing_flag() {
    let dir = tempfile::tempdir().unwrap();
    
    // Create source and destination files
    let source_file = dir.path().join("source.txt");
    let dest_file = dir.path().join("dest.txt");
    fs::write(&source_file, "source content").unwrap();
    fs::write(&dest_file, "dest content").unwrap();

    symlink_binary("mv", dir.path());

    // Move with --backup=existing flag (no existing backup)
    let mut cmd = Command::cargo_bin("smartfo").unwrap();
    cmd.arg("--backup=existing")
        .arg("source.txt")
        .arg("dest.txt")
        .current_dir(dir.path())
        .assert()
        .success();

    // Verify source is gone
    assert!(!source_file.exists());
    // Verify dest was overwritten
    assert_eq!(fs::read_to_string(&dest_file).unwrap(), "source content");
    // Verify no backup created (none existed before)
    let backup_file = dir.path().join("dest.txt~");
    assert!(!backup_file.exists());
}

/// Test -b flag (short for --backup)
#[test]
fn test_mv_backup_short_flag() {
    let dir = tempfile::tempdir().unwrap();
    
    // Create source and destination files
    let source_file = dir.path().join("source.txt");
    let dest_file = dir.path().join("dest.txt");
    fs::write(&source_file, "source content").unwrap();
    fs::write(&dest_file, "dest content").unwrap();

    symlink_binary("mv", dir.path());

    // Move with -b flag
    let mut cmd = Command::cargo_bin("smartfo").unwrap();
    cmd.arg("-b")
        .arg("source.txt")
        .arg("dest.txt")
        .current_dir(dir.path())
        .assert()
        .success();

    // Verify source is gone
    assert!(!source_file.exists());
    // Verify dest was overwritten
    assert_eq!(fs::read_to_string(&dest_file).unwrap(), "source content");
    // Verify backup file exists
    let backup_file = dir.path().join("dest.txt~");
    assert!(backup_file.exists());
}

/// Test default behavior: overwrite without prompt
#[test]
fn test_mv_default_overwrite() {
    let dir = tempfile::tempdir().unwrap();
    
    // Create source and destination files
    let source_file = dir.path().join("source.txt");
    let dest_file = dir.path().join("dest.txt");
    fs::write(&source_file, "source content").unwrap();
    fs::write(&dest_file, "dest content").unwrap();

    symlink_binary("mv", dir.path());

    // Move without any flags (default behavior)
    let mut cmd = Command::cargo_bin("smartfo").unwrap();
    cmd.arg("source.txt")
        .arg("dest.txt")
        .current_dir(dir.path())
        .assert()
        .success();

    // Verify source is gone
    assert!(!source_file.exists());
    // Verify dest was overwritten
    assert_eq!(fs::read_to_string(&dest_file).unwrap(), "source content");
}
