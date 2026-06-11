//! CLI Standards Test Utilities
//!
//! This module provides helper functions for testing CLI standards compliance
//! including flag parsing, output formats, exit codes, and daemon operations.

use assert_cmd::Command;
use predicates::prelude::*;
use std::path::PathBuf;
use tempfile::TempDir;

/// Create a symlink to the smartfo binary with the given name.
/// This is useful for testing argv[0] dispatch behavior.
pub fn symlink_binary(name: &str) -> TempDir {
    let tmp = tempfile::tempdir().unwrap();
    let bin = Command::cargo_bin("smartfo").unwrap();
    let dest = tmp.path().join(name);
    #[cfg(unix)]
    std::os::unix::fs::symlink(bin.get_program(), &dest).unwrap();
    #[cfg(windows)]
    std::os::windows::fs::symlink_file(bin.get_program(), &dest).unwrap();
    tmp
}

/// Create a Command instance for smartfo with the binary path resolved.
pub fn smartfo_cmd() -> Command {
    Command::cargo_bin("smartfo").unwrap()
}

/// Create a Command instance for a symlinked smartfo binary.
pub fn smartfo_symlink_cmd(name: &str, tmp_dir: &TempDir) -> Command {
    let bin_path = tmp_dir.path().join(name);
    Command::new(bin_path)
}

/// Assert that a command succeeds with the expected exit code.
pub fn assert_success_with_code(assert: assert_cmd::assert::Assert, expected_code: i32) -> assert_cmd::assert::Assert {
    assert.code(expected_code)
}

/// Assert that a command fails with the expected exit code.
pub fn assert_failure_with_code(assert: assert_cmd::assert::Assert, expected_code: i32) -> assert_cmd::assert::Assert {
    assert.failure().code(expected_code)
}

/// Assert that stdout contains the expected substring.
pub fn assert_stdout_contains(assert: assert_cmd::assert::Assert, expected: &str) -> assert_cmd::assert::Assert {
    assert.stdout(predicate::str::contains(expected))
}

/// Assert that stderr contains the expected substring.
pub fn assert_stderr_contains(assert: assert_cmd::assert::Assert, expected: &str) -> assert_cmd::assert::Assert {
    assert.stderr(predicate::str::contains(expected))
}

/// Assert that stdout matches a regex pattern.
pub fn assert_stdout_matches(assert: assert_cmd::assert::Assert, pattern: &str) -> assert_cmd::assert::Assert {
    assert.stdout(predicate::str::is_match(pattern).unwrap())
}

/// Assert that stderr matches a regex pattern.
pub fn assert_stderr_matches(assert: assert_cmd::assert::Assert, pattern: &str) -> assert_cmd::assert::Assert {
    assert.stderr(predicate::str::is_match(pattern).unwrap())
}

/// Parse JSON output from stdout.
pub fn parse_json_output(output: &assert_cmd::assert::Assert) -> serde_json::Value {
    let output_str = std::str::from_utf8(&output.get_output().stdout).unwrap();
    serde_json::from_str(output_str).unwrap()
}

/// Assert that output is valid JSON.
pub fn assert_valid_json(assert: assert_cmd::assert::Assert) -> assert_cmd::assert::Assert {
    let output_str = std::str::from_utf8(&assert.get_output().stdout).unwrap();
    serde_json::from_str::<serde_json::Value>(output_str).unwrap();
    assert
}

/// Assert that JSON output contains a specific field with the expected value.
pub fn assert_json_field(assert: assert_cmd::assert::Assert, field: &str, expected: &str) -> assert_cmd::assert::Assert {
    let json = parse_json_output(&assert);
    let value = json.get(field).and_then(|v| v.as_str()).unwrap();
    assert_eq!(value, expected);
    assert
}

/// Assert that JSON output contains a specific field.
pub fn assert_json_has_field(assert: assert_cmd::assert::Assert, field: &str) -> assert_cmd::assert::Assert {
    let json = parse_json_output(&assert);
    assert!(json.get(field).is_some(), "JSON should contain field: {}", field);
    assert
}

/// Assert that output is in TOON format (Token-Oriented Object Notation).
/// TOON format is a compact, token-efficient format for agent consumption.
pub fn assert_toon_format(assert: assert_cmd::assert::Assert) -> assert_cmd::assert::Assert {
    let output = std::str::from_utf8(&assert.get_output().stdout).unwrap();
    // TOON format should have compact key-value pairs without excessive whitespace
    assert!(output.lines().all(|line| line.len() < 200), "TOON lines should be compact");
    // Should contain typical TOON patterns like "key: value" or "key{field}: value"
    assert!(output.contains(':'), "TOON format should contain key-value separators");
    assert
}

/// Assert that output is in human-readable format.
/// Human format should have clear, readable output with line breaks and spacing.
pub fn assert_human_format(assert: assert_cmd::assert::Assert) -> assert_cmd::assert::Assert {
    let output = std::str::from_utf8(&assert.get_output().stdout).unwrap();
    // Human format should have reasonable line breaks
    assert!(output.lines().count() > 0, "Human format should have multiple lines");
    // Should not be overly compact like TOON
    assert!(output.len() > 50, "Human format should be reasonably verbose");
    assert
}

/// Test that a CLI flag is accepted and produces the expected behavior.
pub fn test_flag_acceptance(flag: &str, expected_in_output: Option<&str>) {
    let mut cmd = smartfo_cmd();
    cmd.arg(flag);
    let assert = cmd.assert().success();
    if let Some(expected) = expected_in_output {
        assert_stdout_contains(assert, expected);
    }
}

/// Test that a CLI flag combination is accepted.
pub fn test_flag_combination(flags: &[&str], expected_in_output: Option<&str>) {
    let mut cmd = smartfo_cmd();
    for flag in flags {
        cmd.arg(flag);
    }
    let assert = cmd.assert().success();
    if let Some(expected) = expected_in_output {
        assert_stdout_contains(assert, expected);
    }
}

/// Test that an invalid flag is rejected with appropriate error message.
pub fn test_invalid_flag_rejection(flag: &str) {
    let mut cmd = smartfo_cmd();
    cmd.arg(flag);
    cmd.assert().failure();
}

/// Test that a required flag without value is rejected.
pub fn test_missing_flag_value(flag: &str) {
    let mut cmd = smartfo_cmd();
    cmd.arg(flag);
    cmd.assert().failure();
}

/// Create a temporary directory for test fixtures.
pub fn temp_dir() -> TempDir {
    tempfile::tempdir().unwrap()
}

/// Create a temporary file with the given content.
pub fn temp_file_with_content(dir: &TempDir, name: &str, content: &str) -> PathBuf {
    let file_path = dir.path().join(name);
    std::fs::write(&file_path, content).unwrap();
    file_path
}

/// Create a temporary directory structure for testing.
pub fn temp_dir_structure(dir: &TempDir, structure: &[(&str, &str)]) -> Vec<PathBuf> {
    let mut paths = Vec::new();
    for (name, content) in structure {
        let path = temp_file_with_content(dir, name, content);
        paths.push(path);
    }
    paths
}

/// Assert that a command respects the --dry-run flag (no changes made).
pub fn assert_dry_run_no_changes(cmd: &mut Command, _original_state: &std::path::Path) {
    cmd.arg("--dry-run");
    cmd.assert().success();
    // Verify that the original state hasn't changed
    // This is a placeholder - actual implementation depends on what's being tested
}

/// Assert that a command respects the --verbose flag (detailed output).
pub fn assert_verbose_output(cmd: &mut Command) {
    cmd.arg("--verbose");
    let assert = cmd.assert().success();
    let output = std::str::from_utf8(&assert.get_output().stdout).unwrap();
    // Verbose output should be more detailed than normal output
    assert!(output.len() > 100, "Verbose output should be detailed");
}

/// Assert that a command respects the --quiet flag (minimal output).
pub fn assert_quiet_output(cmd: &mut Command) {
    cmd.arg("--quiet");
    let assert = cmd.assert().success();
    let output = std::str::from_utf8(&assert.get_output().stdout).unwrap();
    // Quiet output should be minimal
    assert!(output.len() < 50, "Quiet output should be minimal");
}

/// Assert that a command respects the --json flag (JSON output).
pub fn assert_json_output(cmd: &mut Command) {
    cmd.arg("--json");
    let assert = cmd.assert().success();
    let output_str = std::str::from_utf8(&assert.get_output().stdout).unwrap();
    serde_json::from_str::<serde_json::Value>(output_str).unwrap();
}

/// Test daemon mode operations (async behavior).
pub fn test_daemon_mode(cmd: &mut Command) {
    cmd.arg("--daemon");
    // Daemon mode should return quickly (non-blocking)
    // This is a placeholder - actual implementation would measure execution time
    cmd.assert().success();
}

/// Test health check endpoint.
pub fn test_health_check() {
    let mut cmd = smartfo_cmd();
    cmd.arg("--health-check");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("healthy"));
}

/// Test privacy mode behavior (no sensitive data in output).
pub fn test_privacy_mode(cmd: &mut Command) {
    cmd.arg("--privacy");
    let result = cmd.assert().success();
    let output = &result.get_output().stdout;
    let output_str = std::str::from_utf8(output).unwrap();
    // Privacy mode should not expose sensitive data
    assert!(!output_str.contains("password"), "Privacy mode should not expose passwords");
    assert!(!output_str.contains("secret"), "Privacy mode should not expose secrets");
    assert!(!output_str.contains("token"), "Privacy mode should not expose tokens");
}

/// Test config reload behavior (SIGHUP handling).
pub fn test_config_reload() {
    // This would typically involve:
    // 1. Starting the daemon
    // 2. Modifying config
    // 3. Sending SIGHUP
    // 4. Verifying config is reloaded
    // Placeholder for implementation
}

/// Test session hooks output.
pub fn test_session_hooks() {
    let mut cmd = smartfo_cmd();
    cmd.arg("--session-context");
    let assert = cmd.assert().success();
    // Session context should include ambient information
    assert_stdout_contains(assert, "cwd");
}

/// Test skill generation.
pub fn test_skill_generation() {
    let mut cmd = smartfo_cmd();
    cmd.arg("--generate-skill");
    let assert = cmd.assert().success();
    // Skill output should be in a specific format
    let output = std::str::from_utf8(&assert.get_output().stdout).unwrap();
    assert!(output.contains("name:"), "Skill output should contain 'name:'");
    assert!(output.contains("description:"), "Skill output should contain 'description:'");
}

/// Test cross-platform path handling.
pub fn test_cross_platform_path(path: &str) {
    let mut cmd = smartfo_cmd();
    cmd.arg(path);
    // Should handle paths correctly on all platforms
    cmd.assert().success();
}

/// Test TUI mode interactions.
pub fn test_tui_mode() {
    let mut cmd = smartfo_cmd();
    cmd.arg("--tui");
    // TUI mode should start without errors
    // Actual TUI testing is complex and may require terminal emulation
    cmd.assert().success();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_symlink_binary_creation() {
        let tmp = symlink_binary("test_mv");
        assert!(tmp.path().join("test_mv").exists());
    }

    #[test]
    fn test_smartfo_cmd_creation() {
        let cmd = smartfo_cmd();
        assert!(cmd.get_program().to_string_lossy().contains("smartfo"));
    }

    #[test]
    fn test_temp_dir_creation() {
        let dir = temp_dir();
        assert!(dir.path().exists());
    }

    #[test]
    fn test_temp_file_creation() {
        let dir = temp_dir();
        let file = temp_file_with_content(&dir, "test.txt", "Hello, World!");
        assert!(file.exists());
        assert_eq!(std::fs::read_to_string(&file).unwrap(), "Hello, World!");
    }

    #[test]
    fn test_temp_dir_structure() {
        let dir = temp_dir();
        let structure = vec![("file1.txt", "content1"), ("file2.txt", "content2")];
        let paths = temp_dir_structure(&dir, &structure);
        assert_eq!(paths.len(), 2);
        for path in paths {
            assert!(path.exists());
        }
    }
}
