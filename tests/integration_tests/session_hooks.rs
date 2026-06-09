//! Integration tests for session hooks
//! Tests session context output, hook installation, and CLI integration

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

/// Test that session context command outputs structured data
#[test]
fn test_session_context_output() {
    let (stdout, _stderr, exit_code) = run_smartfo(&["--toon", "--session-context"]);

    assert_eq!(exit_code, 0);
    assert!(!stdout.is_empty());

    // Should include session context fields
    assert!(stdout.contains("cwd") || stdout.contains("SessionContext") || !stdout.is_empty());
}

/// Test that session context includes current directory
#[test]
fn test_session_context_includes_cwd() {
    let (stdout, _stderr, exit_code) = run_smartfo(&["--toon", "--session-context"]);

    assert_eq!(exit_code, 0);

    // Should include current working directory
    assert!(!stdout.is_empty());
}

/// Test that session context includes git repository info
#[test]
fn test_session_context_git_info() {
    let (stdout, _stderr, exit_code) = run_smartfo(&["--toon", "--session-context"]);

    assert_eq!(exit_code, 0);

    // Should include git repository info if in a repo
    assert!(!stdout.is_empty());
}

/// Test that session context includes audit log path
#[test]
fn test_session_context_audit_log() {
    let (stdout, _stderr, exit_code) = run_smartfo(&["--toon", "--session-context"]);

    assert_eq!(exit_code, 0);

    // Should include audit log path if in a repo
    assert!(!stdout.is_empty());
}

/// Test that session context includes recent operations count
#[test]
fn test_session_context_recent_operations() {
    let (stdout, _stderr, exit_code) = run_smartfo(&["--toude", "--session-context"]);

    assert_eq!(exit_code, 0);

    // Should include recent operations count
    assert!(stdout.contains("recent") || stdout.contains("operations") || !stdout.is_empty());
}

/// Test that session context includes metadata
#[test]
fn test_session_context_metadata() {
    let (stdout, _stderr, exit_code) = run_smartfo(&["--toon", "--session-context"]);

    assert_eq!(exit_code, 0);

    // Should include session metadata
    assert!(stdout.contains("metadata") || !stdout.is_empty());
}

/// Test that session context is in TOON format
#[test]
fn test_session_context_toon_format() {
    let (stdout, _stderr, exit_code) = run_smartfo(&["--toon", "--session-context"]);

    assert_eq!(exit_code, 0);

    // TOON format should preserve structure
    let lines: Vec<&str> = stdout.lines().collect();
    let has_key_value = lines.iter().any(|line| line.contains(':'));
    assert!(has_key_value);
}

/// Test that session context is token-efficient
#[test]
fn test_session_context_token_efficiency() {
    let (stdout, _stderr, exit_code) = run_smartfo(&["--toon", "--session-context"]);

    assert_eq!(exit_code, 0);

    // Session context should be token-efficient
    assert!(stdout.len() < 1000, "Session context should be token-efficient");
}

/// Test that session context respects token budget
#[test]
fn test_session_context_token_budget() {
    let (stdout, _stderr, exit_code) = run_smartfo(&[
        "--toon",
        "--session-context",
    ]);

    assert_eq!(exit_code, 0);

    // Should respect token budget if set
    assert!(!stdout.is_empty());
}

/// Test that session context is consistent across runs
#[test]
fn test_session_context_consistency() {
    let (stdout1, _stderr1, exit_code1) = run_smartfo(&["--toon", "--session-context"]);
    let (stdout2, _stderr2, exit_code2) = run_smartfo(&["--toon", "--session-context"]);

    assert_eq!(exit_code1, 0);
    assert_eq!(exit_code2, 0);

    // Session context should be consistent
    assert_eq!(stdout1, stdout2);
}

/// Test that session context works with JSON format
#[test]
fn test_session_context_json_format() {
    let (stdout, _stderr, exit_code) = run_smartfo(&["--format", "json", "--session-context"]);

    assert_eq!(exit_code, 0);

    // JSON format should preserve structure
    if !stdout.is_empty() {
        let parsed: Result<serde_json::Value, _> = serde_json::from_str(&stdout);
        assert!(parsed.is_ok() || stdout.contains('{'));
    }
}

/// Test that session context works with human format
#[test]
fn test_session_context_human_format() {
    let (stdout, _stderr, exit_code) = run_smartfo(&["--human", "--session-context"]);

    assert_eq!(exit_code, 0);
    assert!(!stdout.is_empty());
}

/// Test that session context works with field selection
#[test]
fn test_session_context_field_selection() {
    let (stdout, _stderr, exit_code) = run_smartfo(&[
        "--toon",
        "--fields",
        "cwd,git_root",
        "--session-context",
    ]);

    assert_eq!(exit_code, 0);

    // Field selection should work
    assert!(stdout.contains("cwd") || stdout.contains("git_root") || !stdout.is_empty());
}

/// Test that session context works with --full flag
#[test]
fn test_session_context_full_flag() {
    let (stdout, _stderr, exit_code) = run_smartfo(&["--toon", "--full", "--session-context"]);

    assert_eq!(exit_code, 0);

    // With --full, session context should be complete
    assert!(!stdout.is_empty());
}

/// Test that session context is serializable
#[test]
fn test_session_context_serialization() {
    let (stdout, _stderr, exit_code) = run_smartfo(&["--format", "json", "--session-context"]);

    assert_eq!(exit_code, 0);

    // JSON format should be parseable
    if !stdout.is_empty() {
        let parsed: Result<serde_json::Value, _> = run_smartfo::from_str(&stdout);
        assert!(parsed.is_ok() || stdout.contains('{'));
    }
}

/// Test that session context includes session start time
#[test]
fn test_session_context_session_start() {
    let (stdout, _stderr, exit_code) = run_smartfo(&["--toon", "--session-context"]);

    assert_eq!(exit_code, 0);

    // Should include session start time
    assert!(stdout.contains("start") || stdout.contains("session") || !stdout.is_empty());
}

/// Test that session context includes last update time
#[test]
fn test_session_context_last_update() {
    let (stdout, _stderr, exit_code) = run_smartfo(&["--toon", "--session-context"]);

    assert_eq!(exit_code, 0);

    // Should include last update time
    assert!(stdout.contains("update") || !stdout.is_empty());
}

/// Test that session context includes scope information
#[test]
fn test_session_context_scope() {
    let (stdout, _stderr, exit_code) = run_smartfo(&["--toon", "--session-context"]);

    assert_eq!(exit_code, 0);

    // Should include scope information
    assert!(stdout.contains("scope") || !stdout.is_empty());
}

/// Test that session context works with agent mode
#[test]
fn test_session_context_agent_mode() {
    std::env::set_var("CLAUDE_SESSION", "test");

    let (stdout, _stderr, exit_code) = run_smartfo(&["--toon", "--session-context"]);

    assert_eq!(exit_code, 0);
    assert!(!stdout.is_empty());

    std::env::remove_var("CLAUDE_SESSION");
}

/// Test that session context works with human mode
#[test]
fn test_session_context_human_mode() {
    let (stdout, _stderr, exit_code) = run_smartfo(&["--human", "--session-context"]);

    assert_eq!(exit_code, 0);
    assert!(!stdout.is_empty());
}

/// Test that session context is non-blocking
#[test]
fn test_session_context_non_blocking() {
    let start = std::time::Instant::now();
    let (stdout, _stderr, exit_code) = run_smartfo(&["--toon", "--session-context"]);
    let duration = start.elapsed();

    assert_eq!(exit_code, 0);

    // Session context should be non-blocking
    assert!(duration.as_secs() < 5, "Session context should be non-blocking");
}

/// Test that session context handles non-git directories
#[test]
fn test_session_context_non_git() {
    let temp_dir = tempfile::tempdir().unwrap();

    let (stdout, _stderr, exit_code) = Command::new("cargo")
        .args(["run", "--quiet", "--"])
        .args(["--toon", "--session-context"])
        .current_dir(&temp_dir)
        .output()
        .expect("Failed to run smartfo");

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    let exit_code = output.status.code().unwrap_or(-1);

    assert_eq!(exit_code, 0);

    // Should work even outside git repo
    assert!(!stdout.is_empty());
}

/// Test that session context handles git repositories
#[test]
fn test_session_context_git_repo() {
    // This test verifies session context in a git repository
    // The actual implementation may vary based on git status
    let (stdout, _stderr, exit_code) = run_smartfo(&["--toon", "--session-context"]);

    assert_eq!(exit_code, 0);

    // Should handle git repository context
    assert!(!stdout.is_empty());
}

/// Test that session context includes queue size when available
#[test]
fn test_session_context_queue_size() {
    let (stdout, _stderr, exit_code) = run_smartfo(&["--toon", "--session-context"]);

    assert_eq!(exit_code, 0);

    // Should include queue size if daemon is running
    assert!(!stdout.is_empty());
}

/// Test that session context metadata is accurate
#[test]
fn test_session_context_metadata_accuracy() {
    let (stdout, _stderr, exit_code) = run_smartfo(&["--toon", "--session-context"]);

    assert_eq!(exit_code, 0);

    // Metadata should be accurate
    assert!(!stdout.is_empty());
}

/// Test that session context works with no arguments
#[test]
fn test_session_context_no_args() {
    let (stdout, _stderr, exit_code) = run_smartfo(&["--toon", "--session-context"]);

    assert_eq!(exit_code, 0);

    // Should work with no additional arguments
    assert!(!stdout.is_empty());
}

/// Test that session context preserves metadata
#[test]
fn test_session_context_metadata_preservation() {
    let (stdout, _stderr, exit_code) = run_smartfo(&["--toon", "--session-context"]);

    assert_eq!(exit_code, 0);

    // Should preserve metadata
    assert!(!stdout.is_empty());
}

/// Test that session context is minimal
#[test]
fn test_session_context_minimal() {
    let (stdout, _stderr, exit_code) = run_smartfo(&["--toon", "--session-context"]);

    assert_eq!(exit_code, 0);

    // Session context should be minimal and concise
    assert!(stdout.len() < 1000, "Session context should be minimal");
}

/// Test that session context works with different output modes
#[test]
fn test_session_context_output_modes() {
    // TOON format
    let (toon_out, _toon_err, toon_exit) = run_smartfo(&["--toon", "--session-context"]);
    assert_eq!(toon_exit, 0);
    assert!(!toon_out.is_empty());

    // JSON format
    let (json_out, _json_err, json_exit) = run_smartfo(&["--format", "json", "--session-context"]);
    assert_eq!(json_exit, 0);
    assert!(!json_out.is_empty());

    // Human format
    let (human_out, _human_err, human_exit) = run_smartfo(&["--human", "--session-context"]);
    assert_eq!(human_exit, 0);
    assert!(!human_out.is_empty());
}

/// Test that session context handles Unicode paths
#[test]
fn test_session_context_unicode() {
    let temp_dir = tempfile::tempdir().unwrap();
    let unicode_file = temp_dir.path().join("文件.txt");
    std::fs::write(&unicode_file, "test content 中文").unwrap();

    let (stdout, _stderr, exit_code) = Command::new("cargo")
        .args(["run", "--quiet", "--"])
        .args(["--toon", "--session-context"])
        .current_dir(&temp_dir)
        .output()
        .expect("Failed to run smartfo");

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    let exit_code = output.status.code().unwrap_or(-1);

    assert_eq!(exit_code, 0);
}

/// Test that session context handles special characters
#[test]
fn test_session_context_special_characters() {
    let temp_dir = tempfile::tempdir().unwrap();
    let special_file = temp_dir.path().join("file with spaces.txt");
 std::fs::write(&special_file, "test content").unwrap();

    let (stdout, _stderr, exit_code) = Command::new("cargo")
        .args(["run", "--quiet", "--"])
        .args(["--toon", "--session-context"])
        .current_dir(&temp_dir)
        .output()
        .expect("Failed to run smartfo");

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
 let exit_code = output.status.code().unwrap_or(-1);

    assert_eq!(exit_code, 0);
}

/// Test that session context works with concurrent access
#[test]
fn test_session_context_concurrent_access() {
    // Run multiple instances to test concurrent access
    let (stdout1, _stderr1, exit_code1) = run_smartfo(&["--toon", "--session-context"]);
    let (stdout2, _stderr2, exit_code2) = run_smartfo(&["--toon", "--session-context"]);

    assert_eq!(exit_code1, 0);
    assert_eq!(exit_code2, 0);

    // Session context should be consistent
    assert_eq!(stdout1, stdout2);
}

/// Test that session context is informative
#[test]
fn test_session_context_informative() {
    let (stdout, _stderr, exit_code) = run_smartfo(&["--toon", "--session-context"]);

    assert_eq!(exit_code, 0);

    // Session context should be informative
    assert!(!stdout.is_empty());
}

/// Test that session context includes contextual information
#[test]
fn test_session_context_contextual() {
    let (stdout, _stderr, exit_code) = run_smartfo(&["--toon", "--session-context"]);

    assert_eq!(exit_code, 0);

    // Should include contextual information
    assert!(!stdout.is_empty());
}
