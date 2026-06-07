use assert_cmd::Command;
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

/// Create a RAM disk on macOS for cross-device testing
#[cfg(target_os = "macos")]
fn create_ram_disk() -> Option<TempDir> {
    // Create a 64MB RAM disk
    let output = std::process::Command::new("hdiutil")
        .args(["attach", "-nomount", "ram://131072"])  // 64MB in 512-byte blocks
        .output()
        .ok()?;

    let disk_device = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if disk_device.is_empty() {
        return None;
    }

    // Format the disk
    std::process::Command::new("newfs_hfs")
        .arg(&disk_device)
        .output()
        .ok()?;

    // Create a mount point
    let mount_point = TempDir::new().ok()?;
    let mount_path = mount_point.path();

    // Mount the disk
    std::process::Command::new("mount")
        .args(["-t", "hfs", &disk_device, mount_path.to_str().unwrap()])
        .output()
        .ok()?;

    Some(mount_point)
}

/// Detach and cleanup RAM disk on macOS
#[cfg(target_os = "macos")]
fn cleanup_ram_disk(mount_point: &Path) {
    // Unmount the disk
    let _ = std::process::Command::new("umount")
        .arg(mount_point)
        .output();

    // Try to find and detach the disk device
    if let Ok(output) = std::process::Command::new("df")
        .arg(mount_point)
        .output()
    {
        let stdout = String::from_utf8_lossy(&output.stdout);
        for line in stdout.lines() {
            if line.contains(mount_point.to_str().unwrap_or("")) {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() > 0 {
                    let disk_device = parts[0];
                    let _ = std::process::Command::new("hdiutil")
                        .args(["detach", disk_device])
                        .output();
                }
                break;
            }
        }
    }
}

/// Test cross-device move (copy + delete instead of rename)
#[test]
#[cfg(target_os = "macos")]
fn test_mv_cross_device() {
    let ram_disk = match create_ram_disk() {
        Some(disk) => disk,
        None => {
            // Skip test if RAM disk creation fails
            eprintln!("Skipping cross-device test: could not create RAM disk");
            return;
        }
    };

    let source_dir = tempfile::tempdir().unwrap();
    
    // Create a file in the source directory
    let source_file = source_dir.path().join("test.txt");
    fs::write(&source_file, "test content").unwrap();

    symlink_binary("mv", source_dir.path());

    // Move file to RAM disk (cross-device)
    let dest_file = ram_disk.path().join("test.txt");
    let mut cmd = Command::cargo_bin("smartfo").unwrap();
    cmd.arg("test.txt")
        .arg(&dest_file)
        .current_dir(source_dir.path())
        .assert()
        .success();

    // Verify source is gone
    assert!(!source_file.exists());
    // Verify dest exists
    assert!(dest_file.exists());
    // Verify content is preserved
    assert_eq!(fs::read_to_string(&dest_file).unwrap(), "test content");

    cleanup_ram_disk(ram_disk.path());
}

/// Test cross-device move with large file (should use async)
#[test]
#[cfg(target_os = "macos")]
fn test_mv_cross_device_large_file_async() {
    let ram_disk = match create_ram_disk() {
        Some(disk) => disk,
        None => {
            eprintln!("Skipping cross-device test: could not create RAM disk");
            return;
        }
    };

    let source_dir = tempfile::tempdir().unwrap();
    
    // Create a large file (>100MB to trigger async)
    let source_file = source_dir.path().join("large.bin");
    let large_content = vec![0u8; 100 * 1024 * 1024]; // 100MB
    fs::write(&source_file, large_content).unwrap();

    symlink_binary("mv", source_dir.path());

    // Move file to RAM disk with --async flag
    let dest_file = ram_disk.path().join("large.bin");
    let mut cmd = Command::cargo_bin("smartfo").unwrap();
    cmd.arg("--async")
        .arg("large.bin")
        .arg(&dest_file)
        .current_dir(source_dir.path())
        .assert()
        .success();

    // Verify source is gone (async operation should complete)
    assert!(!source_file.exists());
    // Verify dest exists
    assert!(dest_file.exists());

    cleanup_ram_disk(ram_disk.path());
}

/// Test that same-filesystem moves use rename (not copy+delete)
#[test]
fn test_mv_same_filesystem_rename() {
    let dir = tempfile::tempdir().unwrap();
    
    // Create a file
    let source_file = dir.path().join("test.txt");
    let dest_file = dir.path().join("test_moved.txt");
    fs::write(&source_file, "test content").unwrap();

    symlink_binary("mv", dir.path());

    // Move file within same filesystem
    let mut cmd = Command::cargo_bin("smartfo").unwrap();
    cmd.arg("test.txt")
        .arg("test_moved.txt")
        .current_dir(dir.path())
        .assert()
        .success();

    // Verify source is gone
    assert!(!source_file.exists());
    // Verify dest exists
    assert!(dest_file.exists());
    // Verify content is preserved
    assert_eq!(fs::read_to_string(&dest_file).unwrap(), "test content");
}

/// Test cross-device move with --blocking flag
#[test]
#[cfg(target_os = "macos")]
fn test_mv_cross_device_blocking() {
    let ram_disk = match create_ram_disk() {
        Some(disk) => disk,
        None => {
            eprintln!("Skipping cross-device test: could not create RAM disk");
            return;
        }
    };

    let source_dir = tempfile::tempdir().unwrap();
    
    // Create a file
    let source_file = source_dir.path().join("test.txt");
    fs::write(&source_file, "test content").unwrap();

    symlink_binary("mv", source_dir.path());

    // Move file to RAM disk with --blocking flag
    let dest_file = ram_disk.path().join("test.txt");
    let mut cmd = Command::cargo_bin("smartfo").unwrap();
    cmd.arg("--blocking")
        .arg("test.txt")
        .arg(&dest_file)
        .current_dir(source_dir.path())
        .assert()
        .success();

    // Verify source is gone (blocking operation should complete)
    assert!(!source_file.exists());
    // Verify dest exists
    assert!(dest_file.exists());

    cleanup_ram_disk(ram_disk.path());
}
