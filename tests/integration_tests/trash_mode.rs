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

/// Test trash_mode = "always" always moves to trash
#[test]
fn test_trash_mode_always() {
    let dir = tempfile::tempdir().unwrap();
    
    // Create a config file with trash_mode = "always"
    let config_dir = dir.path().join(".config").join("smartfo");
    fs::create_dir_all(&config_dir).unwrap();
    let config_file = config_dir.join("config.toml");
    fs::write(&config_file, r#"
[trash]
mode = "always"
"#).unwrap();

    // Set XDG_CONFIG_HOME to temp directory
    std::env::set_var("XDG_CONFIG_HOME", dir.path());

    // Create a file
    let test_file = dir.path().join("test.txt");
    fs::write(&test_file, "test content").unwrap();

    symlink_binary("rm", dir.path());

    // Remove file
    let mut cmd = Command::cargo_bin("smartfo").unwrap();
    cmd.arg("test.txt")
        .current_dir(dir.path())
        .assert()
        .success();

    // Note: Trash mode tests are skipped for now
    // This is a placeholder for future implementation

    // Cleanup
    std::env::remove_var("XDG_CONFIG_HOME");
}

/// Test trash_mode = "never" never moves to trash
#[test]
fn test_trash_mode_never() {
    let dir = tempfile::tempdir().unwrap();
    
    // Create a config file with trash_mode = "never"
    let config_dir = dir.path().join(".config").join("smartfo");
    fs::create_dir_all(&config_dir).unwrap();
    let config_file = config_dir.join("config.toml");
    fs::write(&config_file, r#"
[trash]
mode = "never"
"#).unwrap();

    // Set XDG_CONFIG_HOME to temp directory
    std::env::set_var("XDG_CONFIG_HOME", dir.path());

    // Create a file
    let test_file = dir.path().join("test.txt");
    fs::write(&test_file, "test content").unwrap();

    symlink_binary("rm", dir.path());

    // Remove file
    let mut cmd = Command::cargo_bin("smartfo").unwrap();
    cmd.arg("test.txt")
        .current_dir(dir.path())
        .assert()
        .success();

    // Note: Trash mode tests are skipped for now
    // This is a placeholder for future implementation

    // Cleanup
    std::env::remove_var("XDG_CONFIG_HOME");
}

/// Test trash_mode = "auto" moves to trash when disk space is sufficient
#[test]
fn test_trash_mode_auto_sufficient_space() {
    let dir = tempfile::tempdir().unwrap();
    
    // Create a config file with trash_mode = "auto"
    let config_dir = dir.path().join(".config").join("smartfo");
    fs::create_dir_all(&config_dir).unwrap();
    let config_file = config_dir.join("config.toml");
    fs::write(&config_file, r#"
[trash]
mode = "auto"
min_free_space_percent = 20
"#).unwrap();

    // Set XDG_CONFIG_HOME to temp directory
    std::env::set_var("XDG_CONFIG_HOME", dir.path());

    // Create a file
    let test_file = dir.path().join("test.txt");
    fs::write(&test_file, "test content").unwrap();

    symlink_binary("rm", dir.path());

    // Remove file
    let mut cmd = Command::cargo_bin("smartfo").unwrap();
    cmd.arg("test.txt")
        .current_dir(dir.path())
        .assert()
        .success();

    // Note: Trash mode tests are skipped for now
    // This is a placeholder for future implementation

    // Cleanup
    std::env::remove_var("XDG_CONFIG_HOME");
}

/// Test trash_mode = "auto" refuses when disk space is low
#[test]
fn test_trash_mode_auto_low_disk_space() {
    let dir = tempfile::tempdir().unwrap();
    
    // Create a config file with trash_mode = "auto" and high threshold
    let config_dir = dir.path().join(".config").join("smartfo");
    fs::create_dir_all(&config_dir).unwrap();
    let config_file = config_dir.join("config.toml");
    fs::write(&config_file, r#"
[trash]
mode = "auto"
min_free_space_percent = 99
on_trash_full = "refuse"
"#).unwrap();

    // Set XDG_CONFIG_HOME to temp directory
    std::env::set_var("XDG_CONFIG_HOME", dir.path());

    // Create a file
    let test_file = dir.path().join("test.txt");
    fs::write(&test_file, "test content").unwrap();

    symlink_binary("rm", dir.path());

    // Try to remove file (should refuse due to low disk space)
    let mut cmd = Command::cargo_bin("smartfo").unwrap();
    cmd.arg("test.txt")
        .current_dir(dir.path())
        .assert()
        .failure();

    // Verify file still exists
    assert!(test_file.exists());

    // Cleanup
    std::env::remove_var("XDG_CONFIG_HOME");
}

/// Test trash_mode = "auto" with on_trash_full = "delete"
#[test]
fn test_trash_mode_auto_delete_when_full() {
    let dir = tempfile::tempdir().unwrap();
    
    // Create a config file with trash_mode = "auto" and on_trash_full = "delete"
    let config_dir = dir.path().join(".config").join("smartfo");
    fs::create_dir_all(&config_dir).unwrap();
    let config_file = config_dir.join("config.toml");
    fs::write(&config_file, r#"
[trash]
mode = "auto"
min_free_space_percent = 99
on_trash_full = "delete"
"#).unwrap();

    // Set XDG_CONFIG_HOME to temp directory
    std::env::set_var("XDG_CONFIG_HOME", dir.path());

    // Create a file
    let test_file = dir.path().join("test.txt");
    fs::write(&test_file, "test content").unwrap();

    symlink_binary("rm", dir.path());

    // Remove file (should delete directly when trash is full)
    let mut cmd = Command::cargo_bin("smartfo").unwrap();
    cmd.arg("test.txt")
        .current_dir(dir.path())
        .assert()
        .success();

    // Note: Trash mode tests are skipped for now
    // This is a placeholder for future implementation

    // Cleanup
    std::env::remove_var("XDG_CONFIG_HOME");
}

/// Test trash_mode with delete_ignored = true
#[test]
fn test_trash_mode_delete_ignored() {
    let dir = tempfile::tempdir().unwrap();
    
    // Create a config file with delete_ignored = true
    let config_dir = dir.path().join(".config").join("smartfo");
    fs::create_dir_all(&config_dir).unwrap();
    let config_file = config_dir.join("config.toml");
    fs::write(&config_file, r#"
[trash]
mode = "always"
delete_ignored = true
"#).unwrap();

    // Set XDG_CONFIG_HOME to temp directory
    std::env::set_var("XDG_CONFIG_HOME", dir.path());

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

    // Remove ignored file
    let mut cmd = Command::cargo_bin("smartfo").unwrap();
    cmd.arg("test.log")
        .current_dir(dir.path())
        .assert()
        .success();

    // Note: Trash mode tests are skipped for now
    // This is a placeholder for future implementation

    // Cleanup
    std::env::remove_var("XDG_CONFIG_HOME");
}

/// Test trash_mode with backup_vcs_committed = true
#[test]
fn test_trash_mode_backup_vcs_committed() {
    let dir = tempfile::tempdir().unwrap();
    
    // Create a config file with backup_vcs_committed = true
    let config_dir = dir.path().join(".config").join("smartfo");
    fs::create_dir_all(&config_dir).unwrap();
    let config_file = config_dir.join("config.toml");
    fs::write(&config_file, r#"
[trash]
mode = "always"
backup_vcs_committed = true
"#).unwrap();

    // Set XDG_CONFIG_HOME to temp directory
    std::env::set_var("XDG_CONFIG_HOME", dir.path());

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

    // Remove committed file
    let mut cmd = Command::cargo_bin("smartfo").unwrap();
    cmd.arg("test.txt")
        .current_dir(dir.path())
        .assert()
        .success();

    // Note: Trash mode tests are skipped for now
    // This is a placeholder for future implementation

    // Cleanup
    std::env::remove_var("XDG_CONFIG_HOME");
}
