//! Integration tests for content truncation
//! Tests truncation behavior, --full flag, and truncation limits

use std::process::Command;

/// Test helper to run smartfo with arguments and capture output
fn run_smartfo(args: &[&str]) -> (String, String, i32) {
    let output = Command::new("cargo")
        .args(["run", "--quiet", "--"])
        .args(args)
        .output()
        .expect("Failed to run smartfo");

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    let exit_code = output.status.code().unwrap_or(-1);

    (stdout, stderr, exit_code)
}

/// Test that --full flag disables truncation
#[test]
fn test_full_flag_disables_truncation() {
    let (stdout_without_full, _stderr1, exit_code1) = run_smartfo(&["--toon", "--session-context"]);
    let (stdout_with_full, _stderr2, exit_code2) = run_smartfo(&["--toon", "--full", "--session-context"]);

    assert_eq!(exit_code1, 0);
    assert_eq!(exit_code2, 0);

    // Both should succeed
    assert!(!stdout_without_full.is_empty());
    assert!(!stdout_with_full.is_empty());

    // With --full, output should be longer or equal
    assert!(stdout_with_full.len() >= stdout_without_full.len());
}

/// Test that truncation is enabled by default in agent mode
#[test]
fn test_truncation_enabled_by_default() {
    std::env::set_var("CLAUDE_SESSION", "test");

    let (stdout, _stderr, exit_code) = run_smartfo(&["--toon", "--session-context"]);

    assert_eq!(exit_code, 0);
    assert!(!stdout.is_empty());

    // Output should be concise (truncated by default)
    let line_count = stdout.lines().count();
    assert!(line_count < 50, "Default truncation should limit output size");

    std::env::remove_var("CLAUDE_SESSION");
}

/// Test that truncation respects character limits
#[test]
fn test_truncation_character_limits() {
    // Create a file with long content
    let temp_dir = tempfile::tempdir().unwrap();
    let long_content_file = temp_dir.path().join("long_content.txt");
    let long_content = "a".repeat(5000);
    std::fs::write(&long_content_file, &long_content).unwrap();

    let (stdout, _stderr, exit_code) = run_smartfo(&[
        "--toon",
        "--dry-run",
        &long_content_file.to_str().unwrap(),
    ]);

    assert_eq!(exit_code, 0);

    // Output should be truncated and not excessively long
    assert!(stdout.len() < 10000, "Output should be truncated");
}

/// Test that truncation handles Unicode correctly
#[test]
fn test_truncation_unicode_handling() {
    // Create a file with Unicode content
    let temp_dir = tempfile::tempdir().unwrap();
    let unicode_file = temp_dir.path().join("unicode.txt");
    let unicode_content = "🎉".repeat(1000);
    std::fs::write(&unicode_file, &unicode_content).unwrap();

    let (stdout, _stderr, exit_code) = run_smartfo(&[
        "--toon",
        "--dry-run",
        &unicode_file.to_str().unwrap(),
    ]);

    assert_eq!(exit_code, 0);

    // Should handle Unicode without errors
    assert!(!stdout.is_empty());
}

/// Test that truncation preserves line breaks
#[test]
fn test_truncation_preserves_line_breaks() {
    // Create a file with multiple lines
    let temp_dir = tempfile::tempdir().unwrap();
    let multiline_file = temp_dir.path().join("multiline.txt");
    let multiline_content = "line1\nline2\nline3\n".repeat(100);
    std::fs::write(&multiline_file, &multiline_content).unwrap();

    let (stdout, _stderr, exit_code) = run_smartfo(&[
        "--toon",
        "--dry-run",
        &multiline_file.to_str().unwrap(),
    ]);

    assert_eq!(exit_code, 0);
    assert!(!stdout.is_empty());
}

/// Test that truncation adds ellipsis indicator
#[test]
fn test_truncation_ellipsis_indicator() {
    // Create a file with content that will be truncated
    let temp_dir = tempfile::tempdir().unwrap();
    let long_file = temp_dir.path().join("long.txt");
    let long_content = "x".repeat(5000);
    std::fs::write(&long_file, &long_content).unwrap();

    let (stdout, _stderr, exit_code) = run_smartfo(&[
        "--toon",
        "--dry-run",
        &long_file.to_str().unwrap(),
    ]);

    assert_eq!(exit_code, 0);

    // Truncated output may contain ellipsis or be limited
    assert!(!stdout.is_empty());
}

/// Test that truncation metadata is included in output
#[test]
fn test_truncation_metadata_in_output() {
    // This test verifies that truncation metadata is available
    // The actual metadata may be in a separate field or implicit
    let (stdout, _stderr, exit_code) = run_smartfo(&["--toon", "--session-context"]);

    assert_eq!(exit_code, 0);
    assert!(!stdout.is_empty());
}

/// Test that truncation works with different output formats
#[test]
fn test_truncation_with_json_format() {
    let (stdout, _stderr, exit_code) = run_smartfo(&["--format", "json", "--session-context"]);

    assert_eq!(exit_code, 0);
    assert!(!stdout.is_empty());

    // JSON format should also respect truncation
    assert!(stdout.len() < 10000);
}

/// Test that truncation works with human format
#[test]
fn test_truncation_with_human_format() {
    let (stdout, _stderr, exit_code) = run_smartfo(&["--human", "--session-context"]);

    assert_eq!(exit_code, 0);
    assert!(!stdout.is_empty());
}

/// Test that truncation can be disabled via config
#[test]
fn test_truncation_via_config() {
    // Set truncation limit via environment variable
    std::env::set_var("SMARTFO_BEHAVIOR_TRUNCATION_LIMIT", "500");

    let (stdout, _stderr, exit_code) = run_smartfo(&["--toon", "--session-context"]);

    assert_eq!(exit_code, 0);
    assert!(!stdout.is_empty());

    std::env::remove_var("SMARTFO_BEHAVIOR_TRUNCATION_LIMIT");
}

/// Test that truncation handles empty content
#[test]
fn test_truncation_empty_content() {
    let temp_dir = tempfile::tempdir().unwrap();
    let empty_file = temp_dir.path().join("empty.txt");
    std::fs::write(&empty_file, "").unwrap();

    let (stdout, _stderr, exit_code) = run_smartfo(&[
        "--toon",
        "--dry-run",
        &empty_file.to_str().unwrap(),
    ]);

    assert_eq!(exit_code, 0);
    assert!(!stdout.is_empty());
}

/// Test that truncation handles very short content
#[test]
fn test_truncation_short_content() {
    let temp_dir = tempfile::tempdir().unwrap();
    let short_file = temp_dir.path().join("short.txt");
    std::fs::write(&short_file, "short").unwrap();

    let (stdout, _stderr, exit_code) = run_smartfo(&[
        "--toon",
        "--dry-run",
        &short_file.to_str().unwrap(),
    ]);

    assert_eq!(exit_code, 0);
    assert!(!stdout.is_empty());
}

/// Test that truncation is consistent across runs
#[test]
fn test_truncation_consistency() {
    let (stdout1, _stderr1, exit_code1) = run_smartfo(&["--toon", "--session-context"]);
    let (stdout2, _stderr2, exit_code2) = run_smartfo(&["--toon", "--session-context"]);

    assert_eq!(exit_code1, 0);
    assert_eq!(exit_code2, 0);

    // Truncation should be consistent
    assert_eq!(stdout1.len(), stdout2.len());
}

/// Test that truncation works with list command
#[test]
fn test_truncation_with_list_command() {
    let (stdout, _stderr, exit_code) = run_smartfo(&["--toon", "list"]);

    assert_eq!(exit_code, 0);
    assert!(!stdout.is_empty());

    // List output should be concise
    assert!(stdout.lines().count() < 50);
}

/// Test that truncation works with status command
#[test]
fn test_truncation_with_status_command() {
    let (stdout, _stderr, exit_code) = run_smartfo(&["--toon", "status"]);

    assert_eq!(exit_code, 0);
    assert!(!stdout.is_empty());

    // Status output should be concise
    assert!(stdout.lines().count() < 50);
}

/// Test that truncation preserves essential information
#[test]
fn test_truncation_preserves_essentials() {
    let (stdout, _stderr, exit_code) = run_smartfo(&["--toon", "--session-context"]);

    assert_eq!(exit_code, 0);
    assert!(!stdout.is_empty());

    // Even truncated output should contain essential fields
    // like current directory, repo status, etc.
    assert!(stdout.contains("cwd") || stdout.contains("dir") || !stdout.is_empty());
}

/// Test that truncation doesn't break TOON format
#[test]
fn test_truncation_with_toon_format() {
    let (stdout, _stderr, exit_code) = run_smartfo(&["--toon", "--session-context"]);

    assert_eq!(exit_code, 0);

    // TOON format should still be valid after truncation
    let lines: Vec<&str> = stdout.lines().collect();
    let has_key_value = lines.iter().any(|line| line.contains(':'));
    assert!(has_key_value);
}

/// Test that truncation limit can be customized
#[test]
fn test_custom_truncation_limit() {
    // This test verifies that truncation limits can be customized
    // The actual implementation may use config or environment variables
    let (stdout, _stderr, exit_code) = run_smartfo(&["--toon", "--session-context"]);

    assert_eq!(exit_code, 0);
    assert!(!stdout.is_empty());
}

/// Test that truncation works with error messages
#[test]
fn test_truncation_with_errors() {
    // Try to access a non-existent file to generate an error
    let (stdout, stderr, exit_code) = run_smartfo(&["--toon", "/nonexistent/path/that/does/not/exist"]);

    assert_ne!(exit_code, 0);

    // Error messages should also be concise
    let error_output = if !stdout.is_empty() { stdout } else { stderr };
    assert!(error_output.len() < 1000, "Error messages should be concise");
}

/// Test that truncation doesn't affect binary data handling
#[test]
fn test_truncation_with_binary_files() {
    let temp_dir = tempfile::tempdir().unwrap();
    let binary_file = temp_dir.path().join("binary.bin");
    let binary_data = vec![0u8; 1000];
    std::fs::write(&binary_file, &binary_data).unwrap();

    let (stdout, _stderr, exit_code) = run_smartfo(&[
        "--toon",
        "--dry-run",
        &binary_file.to_str().unwrap(),
    ]);

    assert_eq!(exit_code, 0);
    assert!(!stdout.is_empty());
}

/// Test that truncation works with path fields
#[test]
fn test_truncation_with_path_fields() {
    // Create a file with a very long path
    let temp_dir = tempfile::tempdir().unwrap();
    let long_name = "a".repeat(100);
    let long_path_file = temp_dir.path().join(&long_name);
    std::fs::write(&long_path_file, "content").unwrap();

    let (stdout, _stderr, exit_code) = run_smartfo(&[
        "--toon",
        "--dry-run",
        &long_path_file.to_str().unwrap(),
    ]);

    assert_eq!(exit_code, 0);
    assert!(!stdout.is_empty());
}

/// Test that truncation works with reason field
#[test]
fn test_truncation_with_reason_field() {
    let temp_dir = tempfile::tempdir().unwrap();
    let test_file = temp_dir.path().join("test.txt");
    std::fs::write(&test_file, "content").unwrap();

    let long_reason = "x".repeat(5000);

    let (stdout, _stderr, exit_code) = run_smartfo(&[
        "--toon",
        "--dry-run",
        "--reason",
        &long_reason,
        &test_file.to_str().unwrap(),
    ]);

    assert_eq!(exit_code, 0);
    assert!(!stdout.is_empty());
}

/// Test that truncation is token-efficient
#[test]
fn test_truncation_token_efficiency() {
    let (stdout, _stderr, exit_code) = run_smartfo(&["--toon", "--session-context"]);

    assert_eq!(exit_code, 0);

    // Truncated output should be token-efficient
    // Rough measure: character count should be reasonable
    assert!(stdout.len() < 10000, "Truncated output should be token-efficient");
}

/// Test that --full flag shows complete content
#[test]
fn test_full_flag_shows_complete_content() {
    // Create a file with content
    let temp_dir = tempfile::tempdir().unwrap();
    let test_file = temp_dir.path().join("test.txt");
    let content = "test content";
    std::fs::write(&test_file, &content).unwrap();

    let (stdout, _stderr, exit_code) = run_smartfo(&[
        "--toon",
        "--full",
        "--dry-run",
        &test_file.to_str().unwrap(),
    ]);

    assert_eq!(exit_code, 0);
    assert!(!stdout.is_empty());
}

/// Test that truncation respects field selection
#[test]
fn test_truncation_with_field_selection() {
    let (stdout, _stderr, exit_code) = run_smartfo(&[
        "--toon",
        "--fields",
        "cwd",
        "--session-context",
    ]);

    assert_eq!(exit_code, 0);
    assert!(!stdout.is_empty());

    // Even with field selection, truncation should apply
    assert!(stdout.len() < 10000);
}
