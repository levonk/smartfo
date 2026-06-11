use smartfo::config::{validate_config_file, create_default_config_force};
use std::io::Write;
use tempfile::NamedTempFile;
use std::env;

#[test]
fn test_valid_config_passes_validation() {
    let config_content = r#"
schema_version = "1"

[vcs]
preference = "git"
fallback = ["git", "jj", "hg", "svn"]
supported = ["git", "jj", "hg", "svn"]

[trash]
mode = "versioned"
min_free_mb = 1024
min_free_space_percent = 20
on_trash_full = "refuse"
allow_last_version_cull = false
retention_days = 30
delete_ignored = true
preserve_tree = true
backup_vcs_committed = false

[concurrency]
max_concurrent_jobs = 4
network_limit_mbps = 0
drive_detection = true
network_concurrency = 2

[behavior]
smart_mode = true
async_threshold_mb = 100
default_blocking = false
sync_after_op = false
daemon_fallback_quiet = false
truncation_limit = 1000

[logging]
level = "info"
json = false
color = "auto"

[paths]
audit_log = "/tmp/audit.jsonl"
cache_dir = "/tmp/cache"
"#;

    let mut temp_file = NamedTempFile::new().unwrap();
    temp_file.write_all(config_content.as_bytes()).unwrap();
    temp_file.flush().unwrap();

    let result = validate_config_file(temp_file.path());
    assert!(result.is_ok(), "Valid config should pass validation");
    let config = result.unwrap();
    assert_eq!(config.schema_version, "1");
}

#[test]
fn test_invalid_schema_version_fails() {
    let config_content = r#"
schema_version = "999"

[vcs]
preference = "git"
"#;

    let mut temp_file = NamedTempFile::new().unwrap();
    temp_file.write_all(config_content.as_bytes()).unwrap();
    temp_file.flush().unwrap();

    let result = validate_config_file(temp_file.path());
    assert!(result.is_err(), "Invalid schema version should fail validation");

    let error = result.unwrap_err();
    assert_eq!(error.section, "config");
    assert_eq!(error.key, "schema_version");
    assert!(error.message.contains("Unsupported schema version"));
    assert!(error.suggestion.contains("Update schema_version to 1"));
}

#[test]
fn test_invalid_vcs_preference_fails() {
    let config_content = r#"
schema_version = "1"

[vcs]
preference = "invalid_vcs"
fallback = ["git", "jj", "hg", "svn"]
supported = ["git", "jj", "hg", "svn"]
"#;

    let mut temp_file = NamedTempFile::new().unwrap();
    temp_file.write_all(config_content.as_bytes()).unwrap();
    temp_file.flush().unwrap();

    let result = validate_config_file(temp_file.path());
    assert!(result.is_err(), "Invalid VCS preference should fail validation");

    let error = result.unwrap_err();
    assert_eq!(error.section, "vcs");
    assert_eq!(error.key, "preference");
    assert!(error.message.contains("not in supported list"));
}

#[test]
fn test_invalid_trash_mode_fails() {
    let config_content = r#"
schema_version = "1"

[trash]
mode = "invalid_mode"
"#;

    let mut temp_file = NamedTempFile::new().unwrap();
    temp_file.write_all(config_content.as_bytes()).unwrap();
    temp_file.flush().unwrap();

    let result = validate_config_file(temp_file.path());
    assert!(result.is_err(), "Invalid trash mode should fail validation");

    let error = result.unwrap_err();
    assert_eq!(error.section, "trash");
    assert_eq!(error.key, "mode");
    assert!(error.message.contains("must be 'versioned' or 'simple'"));
}

#[test]
fn test_invalid_min_free_space_percent_fails() {
    let config_content = r#"
schema_version = "1"

[trash]
min_free_space_percent = 150
"#;

    let mut temp_file = NamedTempFile::new().unwrap();
    temp_file.write_all(config_content.as_bytes()).unwrap();
    temp_file.flush().unwrap();

    let result = validate_config_file(temp_file.path());
    assert!(result.is_err(), "Invalid min_free_space_percent should fail validation");

    let error = result.unwrap_err();
    assert_eq!(error.section, "trash");
    assert_eq!(error.key, "min_free_space_percent");
    assert!(error.message.contains("out of range (0-100)"));
}

#[test]
fn test_zero_max_concurrent_jobs_fails() {
    let config_content = r#"
schema_version = "1"

[concurrency]
max_concurrent_jobs = 0
"#;

    let mut temp_file = NamedTempFile::new().unwrap();
    temp_file.write_all(config_content.as_bytes()).unwrap();
    temp_file.flush().unwrap();

    let result = validate_config_file(temp_file.path());
    assert!(result.is_err(), "Zero max_concurrent_jobs should fail validation");

    let error = result.unwrap_err();
    assert_eq!(error.section, "concurrency");
    assert_eq!(error.key, "max_concurrent_jobs");
    assert!(error.message.contains("must be at least 1"));
}

#[test]
fn test_invalid_log_level_fails() {
    let config_content = r#"
schema_version = "1"

[logging]
level = "invalid_level"
"#;

    let mut temp_file = NamedTempFile::new().unwrap();
    temp_file.write_all(config_content.as_bytes()).unwrap();
    temp_file.flush().unwrap();

    let result = validate_config_file(temp_file.path());
    assert!(result.is_err(), "Invalid log level should fail validation");

    let error = result.unwrap_err();
    assert_eq!(error.section, "logging");
    assert_eq!(error.key, "level");
    assert!(error.message.contains("must be one of"));
}

#[test]
fn test_invalid_color_mode_fails() {
    let config_content = r#"
schema_version = "1"

[logging]
color = "invalid_color"
"#;

    let mut temp_file = NamedTempFile::new().unwrap();
    temp_file.write_all(config_content.as_bytes()).unwrap();
    temp_file.flush().unwrap();

    let result = validate_config_file(temp_file.path());
    assert!(result.is_err(), "Invalid color mode should fail validation");

    let error = result.unwrap_err();
    assert_eq!(error.section, "logging");
    assert_eq!(error.key, "color");
    assert!(error.message.contains("must be one of"));
}

#[test]
fn test_large_async_threshold_warns() {
    let config_content = r#"
schema_version = "1"

[behavior]
async_threshold_mb = 20000
"#;

    let mut temp_file = NamedTempFile::new().unwrap();
    temp_file.write_all(config_content.as_bytes()).unwrap();
    temp_file.flush().unwrap();

    let result = validate_config_file(temp_file.path());
    assert!(result.is_err(), "Very large async_threshold_mb should fail validation");

    let error = result.unwrap_err();
    assert_eq!(error.section, "behavior");
    assert_eq!(error.key, "async_threshold_mb");
    assert!(error.message.contains("very large"));
}

#[test]
fn test_zero_truncation_limit_fails() {
    let config_content = r#"
schema_version = "1"

[behavior]
truncation_limit = 0
"#;

    let mut temp_file = NamedTempFile::new().unwrap();
    temp_file.write_all(config_content.as_bytes()).unwrap();
    temp_file.flush().unwrap();

    let result = validate_config_file(temp_file.path());
    assert!(result.is_err(), "Zero truncation_limit should fail validation");

    let error = result.unwrap_err();
    assert_eq!(error.section, "behavior");
    assert_eq!(error.key, "truncation_limit");
    assert!(error.message.contains("must be at least 1"));
}

#[test]
fn test_error_message_formatting() {
    let config_content = r#"
schema_version = "invalid"
"#;

    let mut temp_file = NamedTempFile::new().unwrap();
    temp_file.write_all(config_content.as_bytes()).unwrap();
    temp_file.flush().unwrap();

    let result = validate_config_file(temp_file.path());
    assert!(result.is_err());

    let error = result.unwrap_err();
    let error_string = format!("{}", error);

    // Check that error message contains key components
    assert!(error_string.contains("config"));
    assert!(error_string.contains("schema_version"));
    assert!(error_string.contains("Invalid schema version"));
}

#[test]
fn test_error_suggestion_is_actionable() {
    let config_content = r#"
schema_version = "0"
"#;

    let mut temp_file = NamedTempFile::new().unwrap();
    temp_file.write_all(config_content.as_bytes()).unwrap();
    temp_file.flush().unwrap();

    let result = validate_config_file(temp_file.path());
    assert!(result.is_err());

    let error = result.unwrap_err();

    // Check that suggestion is actionable
    assert!(!error.suggestion.is_empty());
    assert!(error.suggestion.contains("Set") || error.suggestion.contains("Update") || error.suggestion.contains("Remove"));
}

#[test]
fn test_schema_version_validation() {
    // Test valid version
    let config_content = r#"
schema_version = "1"
"#;

    let mut temp_file = NamedTempFile::new().unwrap();
    temp_file.write_all(config_content.as_bytes()).unwrap();
    temp_file.flush().unwrap();

    let result = validate_config_file(temp_file.path());
    assert!(result.is_ok());

    // Test invalid version (zero)
    let config_content = r#"
schema_version = "0"
"#;

    let mut temp_file = NamedTempFile::new().unwrap();
    temp_file.write_all(config_content.as_bytes()).unwrap();
    temp_file.flush().unwrap();

    let result = validate_config_file(temp_file.path());
    assert!(result.is_err());

    // Test invalid version (non-numeric)
    let config_content = r#"
schema_version = "abc"
"#;

    let mut temp_file = NamedTempFile::new().unwrap();
    temp_file.write_all(config_content.as_bytes()).unwrap();
    temp_file.flush().unwrap();

    let result = validate_config_file(temp_file.path());
    assert!(result.is_err());
}

#[test]
fn test_toml_syntax_error() {
    let config_content = r#"
schema_version = "1"

[vcs
preference = "git"
"#;

    let mut temp_file = NamedTempFile::new().unwrap();
    temp_file.write_all(config_content.as_bytes()).unwrap();
    temp_file.flush().unwrap();

    let result = validate_config_file(temp_file.path());
    assert!(result.is_err(), "Invalid TOML syntax should fail validation");

    let error = result.unwrap_err();
    assert_eq!(error.section, "toml");
    assert_eq!(error.key, "parse");
    assert!(error.message.contains("Invalid TOML syntax"));
}

#[test]
fn test_missing_file_error() {
    let temp_file = NamedTempFile::new().unwrap();
    let path = temp_file.path().to_path_buf();
    // Delete the file so it doesn't exist
    drop(temp_file);

    let result = validate_config_file(&path);
    assert!(result.is_err(), "Missing file should fail validation");

    let error = result.unwrap_err();
    assert_eq!(error.section, "file");
    assert_eq!(error.key, "read");
    assert!(error.message.contains("Failed to read config file"));
}

#[test]
fn test_init_config_creates_valid_config() {
    let tmpdir = tempfile::TempDir::new().unwrap();
    let config_dir = tmpdir.path().join(".config").join("smartfo");
    std::fs::create_dir_all(&config_dir).unwrap();

    // Temporarily override environment variables
    let original_home = env::var("HOME");
    let original_xdg = env::var("XDG_CONFIG_HOME");
    env::set_var("HOME", tmpdir.path());
    env::set_var("XDG_CONFIG_HOME", tmpdir.path().join(".config"));

    // Create config
    let result = create_default_config_force(false);
    assert!(result.is_ok());

    let config_path = result.unwrap();
    assert!(config_path.exists());

    // Validate the created config
    let validation_result = validate_config_file(&config_path);
    assert!(validation_result.is_ok(), "Created config should be valid");

    // Restore original environment
    if let Ok(home) = original_home {
        env::set_var("HOME", home);
    } else {
        env::remove_var("HOME");
    }
    if let Ok(xdg) = original_xdg {
        env::set_var("XDG_CONFIG_HOME", xdg);
    } else {
        env::remove_var("XDG_CONFIG_HOME");
    }
}

#[test]
fn test_init_config_without_force_fails_on_existing() {
    let tmpdir = tempfile::TempDir::new().unwrap();
    let config_dir = tmpdir.path().join(".config").join("smartfo");
    std::fs::create_dir_all(&config_dir).unwrap();

    // Temporarily override environment variables
    let original_home = env::var("HOME");
    let original_xdg = env::var("XDG_CONFIG_HOME");
    env::set_var("HOME", tmpdir.path());
    env::set_var("XDG_CONFIG_HOME", tmpdir.path().join(".config"));

    // Create initial config
    let result = create_default_config_force(false);
    assert!(result.is_ok());

    // Try to recreate without force - should fail
    let result = create_default_config_force(false);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("already exists"));

    // Restore original environment
    if let Ok(home) = original_home {
        env::set_var("HOME", home);
    } else {
        env::remove_var("HOME");
    }
    if let Ok(xdg) = original_xdg {
        env::set_var("XDG_CONFIG_HOME", xdg);
    } else {
        env::remove_var("XDG_CONFIG_HOME");
    }
}

#[test]
fn test_init_config_with_force_overwrites_existing() {
    let tmpdir = tempfile::TempDir::new().unwrap();
    let config_dir = tmpdir.path().join(".config").join("smartfo");
    std::fs::create_dir_all(&config_dir).unwrap();
    let config_path = config_dir.join("config.toml");

    // Temporarily override environment variables
    let original_home = env::var("HOME");
    let original_xdg = env::var("XDG_CONFIG_HOME");
    env::set_var("HOME", tmpdir.path());
    env::set_var("XDG_CONFIG_HOME", tmpdir.path().join(".config"));

    // Create initial config
    let result = create_default_config_force(false);
    assert!(result.is_ok());

    // Write custom content
    std::fs::write(&config_path, "# custom config").unwrap();

    // Recreate with force - should succeed
    let result = create_default_config_force(true);
    assert!(result.is_ok());

    // Verify it was overwritten and is still valid
    let validation_result = validate_config_file(&config_path);
    assert!(validation_result.is_ok(), "Overwritten config should be valid");

    // Restore original environment
    if let Ok(home) = original_home {
        env::set_var("HOME", home);
    } else {
        env::remove_var("HOME");
    }
    if let Ok(xdg) = original_xdg {
        env::set_var("XDG_CONFIG_HOME", xdg);
    } else {
        env::remove_var("XDG_CONFIG_HOME");
    }
}

#[test]
fn test_init_config_with_xdg_config_home() {
    let tmpdir = tempfile::TempDir::new().unwrap();
    let config_dir = tmpdir.path().join("custom_config").join("smartfo");
    std::fs::create_dir_all(&config_dir).unwrap();

    // Temporarily override environment variables
    let original_home = env::var("HOME");
    let original_xdg = env::var("XDG_CONFIG_HOME");
    env::set_var("HOME", tmpdir.path());
    env::set_var("XDG_CONFIG_HOME", tmpdir.path().join("custom_config"));

    // Create config with custom XDG_CONFIG_HOME
    let result = create_default_config_force(false);
    assert!(result.is_ok());

    let config_path = result.unwrap();
    assert!(config_path.exists());
    assert!(config_path.starts_with(tmpdir.path().join("custom_config")));

    // Validate the created config
    let validation_result = validate_config_file(&config_path);
    assert!(validation_result.is_ok(), "Created config should be valid");

    // Restore original environment
    if let Ok(home) = original_home {
        env::set_var("HOME", home);
    } else {
        env::remove_var("HOME");
    }
    if let Ok(xdg) = original_xdg {
        env::set_var("XDG_CONFIG_HOME", xdg);
    } else {
        env::remove_var("XDG_CONFIG_HOME");
    }
}

#[test]
fn test_init_config_with_home_fallback() {
    let tmpdir = tempfile::TempDir::new().unwrap();
    let config_dir = tmpdir.path().join("smartfo");
    std::fs::create_dir_all(&config_dir).unwrap();

    // Temporarily override HOME only (no XDG_CONFIG_HOME)
    let original_home = env::var("HOME");
    let original_xdg = env::var("XDG_CONFIG_HOME");
    env::set_var("HOME", tmpdir.path());
    env::remove_var("XDG_CONFIG_HOME");

    // Create config with HOME fallback
    let result = create_default_config_force(false);
    assert!(result.is_ok());

    let config_path = result.unwrap();
    assert!(config_path.exists());
    assert!(config_path.starts_with(tmpdir.path().join("smartfo")));

    // Validate the created config
    let validation_result = validate_config_file(&config_path);
    assert!(validation_result.is_ok(), "Created config should be valid");

    // Restore original environment
    if let Ok(home) = original_home {
        env::set_var("HOME", home);
    } else {
        env::remove_var("HOME");
    }
    if let Ok(xdg) = original_xdg {
        env::set_var("XDG_CONFIG_HOME", xdg);
    }
}
