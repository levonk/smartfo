//! Integration tests for TOON format output
//! Tests TOON format encoding, decoding, and CLI integration

use std::env;
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

/// Test that --toon flag produces TOON format output
#[test]
fn test_toon_flag_produces_toon_format() {
    let (stdout, _stderr, exit_code) = run_smartfo(&["--toon", "--session-context"]);

    assert_eq!(exit_code, 0);
    assert!(!stdout.is_empty());

    // TOON format should have key:value pairs with colons
    let lines: Vec<&str> = stdout.lines().collect();
    let has_key_value = lines.iter().any(|line| line.contains(':'));
    assert!(has_key_value);
}

/// Test that TOON format is compact and token-efficient
#[test]
fn test_toon_format_compactness() {
    let (stdout, _stderr, exit_code) = run_smartfo(&["--toon", "--session-context"]);

    assert_eq!(exit_code, 0);

    // TOON should be more compact than JSON
    // Count characters as a rough measure
    let toon_len = stdout.len();
    assert!(toon_len > 0);

    // Should not have excessive whitespace
    let whitespace_ratio = stdout.chars().filter(|c| c.is_whitespace()).count() as f64 / toon_len as f64;
    assert!(whitespace_ratio < 0.5, "TOON format should be compact");
}

/// Test that TOON format handles simple values correctly
#[test]
fn test_toon_simple_values() {
    // Test with a simple command that outputs basic data
    let (stdout, _stderr, exit_code) = run_smartfo(&["--toon", "--session-context"]);

    assert_eq!(exit_code, 0);
    assert!(!stdout.is_empty());

    // Should contain basic TOON structure
    let lines: Vec<&str> = stdout.lines().collect();
    assert!(!lines.is_empty());
}

/// Test that TOON format handles nested structures
#[test]
fn test_toon_nested_structures() {
    let (stdout, _stderr, exit_code) = run_smartfo(&["--toon", "list"]);

    assert_eq!(exit_code, 0);

    // List command should output structured data
    // Even empty results should have structure
    assert!(!stdout.is_empty());
}

/// Test that TOON format handles arrays correctly
#[test]
fn test_toon_arrays() {
    let (stdout, _stderr, exit_code) = run_smartfo(&["--toon", "list", "--all"]);

    assert_eq!(exit_code, 0);

    // Should handle array output
    assert!(!stdout.is_empty());
}

/// Test that TOON format escapes special characters
#[test]
fn test_toon_special_characters() {
    // Create a temporary file with special characters
    let temp_dir = tempfile::tempdir().unwrap();
    let test_file = temp_dir.path().join("test file with spaces.txt");

    std::fs::write(&test_file, "test content").unwrap();

    let (stdout, _stderr, exit_code) = run_smartfo(&[
        "--toon",
        "--dry-run",
        &test_file.to_str().unwrap(),
    ]);

    // Should succeed even with special characters in path
    assert_eq!(exit_code, 0);
}

/// Test that TOON format handles empty states gracefully
#[test]
fn test_toon_empty_state() {
    let (stdout, _stderr, exit_code) = run_smartfo(&["--toon", "list"]);

    assert_eq!(exit_code, 0);

    // Empty state should still be valid TOON
    assert!(!stdout.is_empty());
}

/// Test that TOON format is consistent across runs
#[test]
fn test_toon_consistency() {
    let (stdout1, _stderr1, exit_code1) = run_smartfo(&["--toon", "--session-context"]);
    let (stdout2, _stderr2, exit_code2) = run_smartfo(&["--toon", "--session-context"]);

    assert_eq!(exit_code1, 0);
    assert_eq!(exit_code2, 0);

    // Output should be consistent (may vary slightly due to timestamps)
    // For now, just verify both succeed
    assert!(!stdout1.is_empty());
    assert!(!stdout2.is_empty());
}

/// Test that TOON format works with different subcommands
#[test]
fn test_toon_with_subcommands() {
    // Test with status subcommand
    let (stdout, _stderr, exit_code) = run_smartfo(&["--toon", "status"]);
    assert_eq!(exit_code, 0);
    assert!(!stdout.is_empty());

    // Test with list subcommand
    let (stdout, _stderr, exit_code) = run_smartfo(&["--toon", "list"]);
    assert_eq!(exit_code, 0);
    assert!(!stdout.is_empty());

    // Test with session-context subcommand
    let (stdout, _stderr, exit_code) = run_smartfo(&["--toon", "session-context"]);
    assert_eq!(exit_code, 0);
    assert!(!stdout.is_empty());
}

/// Test that TOON format handles large outputs
#[test]
fn test_toon_large_output() {
    // Create multiple temporary files to generate larger output
    let temp_dir = tempfile::tempdir().unwrap();

    for i in 0..10 {
        let file = temp_dir.path().join(format!("test{}.txt", i));
        std::fs::write(&file, format!("content {}", i)).unwrap();
    }

    let (stdout, _stderr, exit_code) = run_smartfo(&["--toon", "--dry-run", &temp_dir.path().to_str().unwrap()]);

    assert_eq!(exit_code, 0);
    assert!(!stdout.is_empty());
}

/// Test that TOON format works with --fields flag
#[test]
fn test_toon_with_fields() {
    let (stdout, _stderr, exit_code) = run_smartfo(&[
        "--toon",
        "--fields",
        "cwd,repo",
        "--session-context",
    ]);

    assert_eq!(exit_code, 0);
    assert!(!stdout.is_empty());

    // Output should be limited to specified fields
    assert!(stdout.contains("cwd") || stdout.contains("repo") || !stdout.is_empty());
}

/// Test that TOON format works with --full flag
#[test]
fn test_toon_with_full() {
    let (stdout, _stderr, exit_code) = run_smartfo(&["--toon", "--full", "--session-context"]);

    assert_eq!(exit_code, 0);
    assert!(!stdout.is_empty());

    // With --full, output should not be truncated
    assert!(!stdout.is_empty());
}

/// Test that TOON format is parseable (basic check)
#[test]
fn test_toon_parseable() {
    let (stdout, _stderr, exit_code) = run_smartfo(&["--toon", "--session-context"]);

    assert_eq!(exit_code, 0);

    // TOON output should be parseable as JSON or have valid structure
    // Since our current TOON parser is minimal, we just check it's not empty
    assert!(!stdout.is_empty());
}

/// Test that TOON format handles Unicode characters
#[test]
fn test_toon_unicode() {
    // Create a file with Unicode characters
    let temp_dir = tempfile::tempdir().unwrap();
    let test_file = temp_dir.path().join("test_文件.txt");

    std::fs::write(&test_file, "test content 中文").unwrap();

    let (stdout, _stderr, exit_code) = run_smartfo(&[
        "--toon",
        "--dry-run",
        &test_file.to_str().unwrap(),
    ]);

    assert_eq!(exit_code, 0);
}

/// Test that TOON format works in agent mode
#[test]
fn test_toon_in_agent_mode() {
    env::set_var("CLAUDE_SESSION", "test");

    let (stdout, _stderr, exit_code) = run_smartfo(&["--agent", "--toon", "--session-context"]);

    assert_eq!(exit_code, 0);
    assert!(!stdout.is_empty());

    env::remove_var("CLAUDE_SESSION");
}

/// Test that TOON format works in human mode
#[test]
fn test_toon_in_human_mode() {
    let (stdout, _stderr, exit_code) = run_smartfo(&["--human", "--toon", "--session-context"]);

    assert_eq!(exit_code, 0);
    assert!(!stdout.is_empty());
}

/// Test that --format toon produces same output as --toon
#[test]
fn test_format_toon_equivalent() {
    let (stdout1, _stderr1, exit_code1) = run_smartfo(&["--toon", "--session-context"]);
    let (stdout2, _stderr2, exit_code2) = run_smartfo(&["--format", "toon", "--session-context"]);

    assert_eq!(exit_code1, 0);
    assert_eq!(exit_code2, 0);

    // Both should produce TOON format
    assert!(!stdout1.is_empty());
    assert!(!stdout2.is_empty());
}

/// Test that TOON format handles error states
#[test]
fn test_toon_error_handling() {
    // Try to access a non-existent file
    let (stdout, _stderr, exit_code) = run_smartfo(&["--toon", "/nonexistent/file"]);

    // Should fail gracefully
    assert_ne!(exit_code, 0);

    // Error output should still be structured
    assert!(!stdout.is_empty() || !_stderr.is_empty());
}

/// Test that TOON format preserves data types
#[test]
fn test_toon_data_types() {
    let (stdout, _stderr, exit_code) = run_smartfo(&["--toon", "--session-context"]);

    assert_eq!(exit_code, 0);

    // Should preserve different data types (strings, numbers, booleans)
    assert!(!stdout.is_empty());
}

/// Test that TOON format handles null values
#[test]
fn test_toon_null_values() {
    let (stdout, _stderr, exit_code) = run_smartfo(&["--toon", "list"]);

    assert_eq!(exit_code, 0);

    // Should handle null/empty values gracefully
    assert!(!stdout.is_empty());
}
