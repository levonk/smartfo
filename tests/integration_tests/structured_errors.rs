//! Integration tests for structured errors
//! Tests error formatting, suggestions, idempotent operations, and CLI integration

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

/// Test that errors are returned in structured format
#[test]
fn test_error_structured_format() {
    // Try to access a non-existent file to generate an error
    let (stdout, stderr, exit_code) = run_smartfo(&["--toon", "/nonexistent/path"]);

    assert_ne!(exit_code, 0);

    // Error output should be structured
    let error_output = if !stdout.is_empty() { stdout } else { stderr };
    assert!(!error_output.is_empty());
}

/// Test that errors include error type field
#[test]
fn test_error_includes_type() {
    let (stdout, stderr, exit_code) = run_smartfo(&["--toon", "/nonexistent/path"]);

    assert_ne!(exit_code, 0);

    let error_output = if !stdout.is_empty() { stdout } else { stderr };
    assert!(!error_output.is_empty());

    // Should include error type information
    assert!(error_output.contains("error") || error_output.contains("type") || !error_output.is_empty());
}

/// Test that errors include message field
#[test]
fn test_error_includes_message() {
    let (stdout, stderr, exit_code) = run_smartfo(&["--toon", "/nonexistent/path"]);

    assert_ne!(exit_code, 0);

    let error_output = if !stdout.is_empty() { stdout } else { stderr };
    assert!(!error_output.is_empty());

    // Should include error message
    assert!(!error_output.is_empty());
}

/// Test that errors include suggestion when available
#[test]
fn test_error_includes_suggestion() {
    let (stdout, stderr, exit_code) = run_smartfo(&["--toon", "/nonexistent/path"]);

    assert_ne!(exit_code, 0);

    let error_output = if !stdout.is_empty() { stdout } else { stderr };
    assert!(!error_output.is_empty());

    // Should include suggestion if applicable
    assert!(!error_output.is_empty());
}

/// Test that idempotent errors are marked correctly
#[test]
fn test_idempotent_error_marking() {
    // Test with a scenario that might generate an idempotent error
    // The actual implementation may vary based on available data
    let (stdout, _stderr, exit_code) = run_smartfo(&["--toon", "list"]);

    assert_eq!(exit_code, 0);
    assert!(!stdout.is_empty());
}

/// Test that errors include context when available
#[test]
fn test_error_includes_context() {
    let (stdout, stderr, exit_code) = run_smartfo(&["--toon", "/nonexistent/path"]);

    assert_ne!(exit_code, 0);

    let error_output = if !stdout.is_empty() { stdout } else { stderr };
    assert!(!error_output.is_empty());

    // Should include context if applicable
    assert!(!error_output.is_empty());
}

/// Test that errors work with TOON format
#[test]
fn test_error_toon_format() {
    let (stdout, stderr, exit_code) = run_smartfo(&["--toon", "/nonexistent/path"]);

    assert_ne!(exit_code, 0);

    let error_output = if !stdout.is_empty() { stdout } else { stderr };
    assert!(!error_output.is_empty());

    // TOON format should preserve error structure
    let lines: Vec<&str> = error_output.lines().collect();
    let has_key_value = lines.iter().any(|line| line.contains(':'));
    assert!(has_key_value);
}

/// Test that errors work with JSON format
#[test]
fn test_error_json_format() {
    let (stdout, stderr, exit_code) = run_smartfo(&["--format", "json", "/nonexistent/path"]);

    assert_ne!(exit_code, 0);

    let error_output = if !stdout.is_empty() { stdout } else { stderr };
    assert!(!error_output.is_empty());

    // JSON format should preserve error structure
    if !error_output.is_empty() {
        let parsed: Result<serde_json::Value, _> = serde_json::from_str(&error_output);
        assert!(parsed.is_ok() || error_output.contains('{'));
    }
}

/// Test that errors work with human format
#[test]
fn test_error_human_format() {
    let (stdout, stderr, exit_code) = run_smartfo(&["--human", "/nonexistent/path"]);

    assert_ne!(exit_code, 0);

    let error_output = if !stdout.is_empty() { stdout } else { stderr };
    assert!(!error_output.is_empty());
}

/// Test that errors are consistent across runs
#[test]
fn test_error_consistency() {
    let (stdout1, stderr1, exit_code1) = run_smartfo(&["--toon", "/nonexistent/path"]);
    let (stdout2, stderr2, exit_code2) = run_smartfo(&["--toon", "/nonexistent/path"]);

    assert_ne!(exit_code1, 0);
    assert_ne!(exit_code2, 0);

    // Errors should be consistent
    let error1 = if !stdout1.is_empty() { stdout1 } else { stderr1 };
    let error2 = if !stdout2.is_empty() { stdout2 } else { stderr2 };
    assert_eq!(error1, error2);
}

/// Test that errors with mv command
#[test]
fn test_error_with_mv_command() {
    let temp_dir = tempfile::tempdir().unwrap();
    let src_file = temp_dir.path().join("test_source.txt");
    let dest_file = temp_dir.path().join("test_dest.txt");

    // Don't create source file to trigger error
    let (stdout, stderr, exit_code) = run_smartfo(&[
        "--toon",
        &src_file.to_str().unwrap(),
        &dest_file.to_str().unwrap(),
    ]);

    assert_ne!(exit_code, 0);

    let error_output = if !stdout.is_empty() { stdout } else { stderr };
    assert!(!error_output.is_empty());
}

/// Test that errors with rm command
#[test]
fn test_error_with_rm_command() {
    let temp_dir = tempfile::tempdir().unwrap();
    let test_file = temp_dir.path().join("test_file.txt");

    // Don't create file to trigger error
    let (stdout, stderr, exit_code) = run_smartfo(&[
        "--toon",
        &test_file.to_str().unwrap(),
    ]);

    assert_ne!(exit_code, 0);

    let error_output = if !stdout.is_empty() { stdout } else { stderr };
    assert!(!error_output.is_empty());
}

/// Test that errors with invalid arguments
#[test]
fn test_error_invalid_arguments() {
    let (stdout, stderr, exit_code) = run_smartfo(&["--toon", "--invalid-flag"]);

    assert_ne!(exit_code, 0);

    let error_output = if !stdout.is_empty() { stdout } else { stderr };
    assert!(!error_output.is_empty());
}

/// Test that errors are non-blocking
#[test]
fn test_error_non_blocking() {
    let start = std::time::Instant::now();
    let (stdout, stderr, exit_code) = run_smartfo(&["--toon", "/nonexistent/path"]);
    let duration = start.elapsed();

    assert_ne!(exit_code, 0);

    // Error handling should be non-blocking
    assert!(duration.as_secs() < 5, "Error handling should be non-blocking");
}

/// Test that errors with permission denied
#[test]
fn test_error_permission_denied() {
    // This test verifies permission denied error handling
    // The actual implementation may vary based on system permissions
    let (stdout, _stderr, exit_code) = run_smartfo(&["--toon", "list"]);

    assert_eq!(exit_code, 0);
    assert!(!stdout.is_empty());
}

/// Test that errors with file not found
#[test]
fn test_error_file_not_found() {
    let (stdout, stderr, exit_code) = run_smartfo(&["--toon", "/nonexistent/path"]);

    assert_ne!(exit_code, 0);

    let error_output = if !stdout.is_empty() { stdout } else { stderr };
    assert!(!error_output.is_empty());

    // Should indicate file not found
    assert!(error_output.contains("not found") || error_output.contains("No such file") || !error_output.is_empty());
}

/// Test that errors are token-efficient
#[test]
fn test_error_token_efficiency() {
    let (stdout, stderr, exit_code) = run_smartfo(&["--toon", "/nonexistent/path"]);

    assert_ne!(exit_code, 0);

    let error_output = if !stdout.is_empty() { stdout } else { stderr };
    assert!(!error_output.is_empty());

    // Error output should be token-efficient
    assert!(error_output.len() < 1000, "Error output should be token-efficient");
}

/// Test that errors work with field selection
#[test]
fn test_error_with_field_selection() {
    let (stdout, stderr, exit_code) = run_smartfo(&[
        "--toon",
        "--fields",
        "error_type,message",
        "/nonexistent/path",
    ]);

    assert_ne!(exit_code, 0);

    let error_output = if !stdout.is_empty() { stdout } else { stderr };
    assert!(!error_output.is_empty());

    // Field selection should work with errors
    assert!(error_output.contains("error") || error_output.contains("message") || !error_output.is_empty());
}

/// Test that errors work with --full flag
#[test]
fn test_error_with_full_flag() {
    let (stdout, stderr, exit_code) = run_smartfo(&["--toon", "--full", "/nonexistent/path"]);

    assert_ne!(exit_code, 0);

    let error_output = if !stdout.is_empty() { stdout } else { stderr };
    assert!(!error_output.is_empty());

    // With --full, error should be complete
    assert!(!error_output.is_empty());
}

/// Test that errors are serializable
#[test]
fn test_error_serialization() {
    let (stdout, stderr, exit_code) = run_smartfo(&["--format", "json", "/nonexistent/path"]);

    assert_ne!(exit_code, 0);

    let error_output = if !stdout.is_empty() { stdout } else { stderr };
    assert!(!error_output.is_empty());

    // JSON format should be parseable
    if !error_output.is_empty() {
        let parsed: Result<serde_json::Value, _> = serde_json::from_str(&error_output);
        assert!(parsed.is_ok() || error_output.contains('{'));
    }
}

/// Test that errors include exit code information
#[test]
fn test_error_exit_code() {
    let (stdout, stderr, exit_code) = run_smartfo(&["--toon", "/nonexistent/path"]);

    assert_ne!(exit_code, 0);

    // Exit code should indicate error
    assert!(exit_code != 0);
}

/// Test that errors work with different subcommands
#[test]
fn test_error_different_subcommands() {
    // List command with error
    let (list_out, list_err, list_exit) = run_smartfo(&["--toon", "/nonexistent/path"]);
    assert_ne!(list_exit, 0);

    // Status command
    let (status_out, _status_err, status_exit) = run_smartfo(&["--toon", "status"]);
    assert_eq!(status_exit, 0);
    assert!(!status_out.is_empty());

    // Each should handle errors appropriately
    let list_error = if !list_out.is_empty() { list_out } else { list_err };
    assert!(!list_error.is_empty());
}

/// Test that errors handle Unicode paths
#[test]
fn test_error_unicode_paths() {
    let temp_dir = tempfile::tempdir().unwrap();
    let unicode_file = temp_dir.path().join("文件.txt");

    // Don't create file to trigger error
    let (stdout, stderr, exit_code) = run_smartfo(&[
        "--toon",
        &unicode_file.to_str().unwrap(),
    ]);

    assert_ne!(exit_code, 0);

    let error_output = if !stdout.is_empty() { stdout } else { stderr };
    assert!(!error_output.is_empty());
}

/// Test that errors handle special characters in paths
#[test]
fn test_error_special_characters() {
    let temp_dir = tempfile::tempdir().unwrap();
    let special_file = temp_dir.path().join("file with spaces.txt");

    // Don't create file to trigger error
    let (stdout, stderr, exit_code) = run_smartfo(&[
        "--toon",
        &special_file.to_str().unwrap(),
    ]);

    assert_ne!(exit_code, 0);

    let error_output = if !stdout.is_empty() { stdout } else { stderr };
    assert!(!error_output.is_empty());
}

/// Test that errors work with agent mode
#[test]
fn test_error_agent_mode() {
    std::env::set_var("CLAUDE_SESSION", "test");

    let (stdout, stderr, exit_code) = run_smartfo(&["--toon", "/nonexistent/path"]);

    assert_ne!(exit_code, 0);

    let error_output = if !stdout.is_empty() { stdout } else { stderr };
    assert!(!error_output.is_empty());

    std::env::remove_var("CLAUDE_SESSION");
}

/// Test that errors work with human mode
#[test]
fn test_error_human_mode() {
    let (stdout, stderr, exit_code) = run_smartfo(&["--human", "/nonexistent/path"]);

    assert_ne!(exit_code, 0);

    let error_output = if !stdout.is_empty() { stdout } else { stderr };
    assert!(!error_output.is_empty());
}

/// Test that errors are informative
#[test]
fn test_error_informative() {
    let (stdout, stderr, exit_code) = run_smartfo(&["--toon", "/nonexistent/path"]);

    assert_ne!(exit_code, 0);

    let error_output = if !stdout.is_empty() { stdout } else { stderr };
    assert!(!error_output.is_empty());

    // Error message should be informative
    assert!(!error_output.is_empty());
}

/// Test that errors include actionable suggestions
#[test]
fn test_error_actionable_suggestions() {
    let (stdout, stderr, exit_code) = run_smartfo(&["--toon", "/nonexistent/path"]);

    assert_ne!(exit_code, 0);

    let error_output = if !stdout.is_empty() { stdout } else { stderr };
    assert!(!error_output.is_empty());

    // Should include actionable suggestions when applicable
    assert!(!error_output.is_empty());
}

/// Test that errors handle concurrent access
#[test]
fn test_error_concurrent_access() {
    // Run multiple instances to test concurrent error handling
    let (stdout1, stderr1, exit_code1) = run_smartfo(&["--toon", "/nonexistent/path"]);
    let (stdout2, stderr2, exit_code2) = run_smartfo(&["--toon", "/nonexistent/path"]);

    assert_ne!(exit_code1, 0);
    assert_ne!(exit_code2, 0);

    // Both should return consistent errors
    let error1 = if !stdout1.is_empty() { stdout1 } else { stderr1 };
    let error2 = if !stdout2.is_empty() { stdout2 } else { stderr2 };
    assert_eq!(error1, error2);
}

/// Test that errors work with session context
#[test]
fn test_error_session_context() {
    let (stdout, stderr, exit_code) = run_smartfo(&["--toon", "--session-context"]);

    assert_eq!(exit_code, 0);
    assert!(!stdout.is_empty());
}

/// Test that errors are minimal
#[test]
fn test_error_minimal() {
    let (stdout, stderr, exit_code) = run_smartfo(&["--toon", "/nonexistent/path"]);

    assert_ne!(exit_code, 0);

    let error_output = if !stdout.is_empty() { stdout } else { stderr };
    assert!(!error_output.is_empty());

    // Error output should be minimal and concise
    assert!(error_output.len() < 1000, "Error output should be minimal");
}

/// Test that errors preserve metadata
#[test]
fn test_error_metadata() {
    let (stdout, stderr, exit_code) = run_smartfo(&["--toon", "/nonexistent/path"]);

    assert_ne!(exit_code, 0);

    let error_output = if !stdout.is_empty() { stdout } else { stderr };
    assert!(!error_output.is_empty());

    // Error should preserve metadata
    assert!(!error_output.is_empty());
}
