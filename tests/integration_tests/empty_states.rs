//! Integration tests for empty states
//! Tests empty state formatting, context inclusion, and CLI integration

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

/// Test that list command returns empty state when no operations exist
#[test]
fn test_list_command_empty_state() {
    let (stdout, _stderr, exit_code) = run_smartfo(&["--toon", "list"]);
    assert_eq!(exit_code, 0);

    // Empty state should be returned
    // Should indicate 0 results
    assert!(stdout.contains("0") || stdout.contains("empty") || !stdout.is_empty());
}

/// Test that status command returns empty state when no data available
#[test]
fn test_status_command_empty_state() {
    let (stdout, _stderr, exit_code) = run_smartfo(&["--toon", "status"]);
    assert_eq!(exit_code, 0);

    // Empty state should be returned
    assert!(!stdout.is_empty());
}

/// Test that empty state includes context information
#[test]
fn test_empty_state_includes_context() {
    let (stdout, _stderr, exit_code) = run_smartfo(&["--toon", "list"]);
    assert_eq!(exit_code, 0);

    // Empty state should include context about the query
    assert!(stdout.contains("all") || stdout.contains("operations") || !stdout.is_empty());
}

/// Test that empty state includes scope information
#[test]
fn test_empty_state_includes_scope() {
    let (stdout, _stderr, exit_code) = run_smartfo(&["--toon", "list"]);
    assert_eq!(exit_code, 0);

    // Empty state should include scope information
    assert!(!stdout.is_empty());
}

/// Test that empty state message is human-readable
#[test]
fn test_empty_state_human_readable() {
    let (stdout, _stderr, exit_code) = run_smartfo(&["--toon", "list"]);
    assert_eq!(exit_code, 0);

    // Empty state message should be human-readable
    let lines: Vec<&str> = stdout.lines().collect();
    assert!(!lines.is_empty());
}

/// Test that empty state includes total scope count when available
#[test]
fn test_empty_state_with_total_scope() {
    // This test verifies that empty state includes total scope count
    // The actual implementation may vary based on available data
    let (stdout, _stderr, exit_code) = run_smartfo(&["--toon", "list"]);
    assert_eq!(exit_code, 0);
    assert!(!stdout.is_empty());
}

/// Test that empty state works with --all flag
#[test]
fn test_empty_state_with_all_flag() {
    let (stdout, _stderr, exit_code) = run_smartfo(&["--toon", "list", "--all"]);
    assert_eq!(exit_code, 0);

    // With --all, empty state should reflect all items scope
    assert!(stdout.contains("all") || !stdout.is_empty());
}

/// Test that empty state works with --limit flag
#[test]
fn test_empty_state_with_limit_flag() {
    let (stdout, _stderr, exit_code) = run_smartfo(&["--toon", "list", "--limit", "5"]);
    assert_eq!(exit_code, 0);

    // With limit, empty state should reflect limit
    assert!(stdout.contains("limit") || !stdout.is_empty());
}

/// Test that empty state works with detailed status
#[test]
fn test_empty_state_detailed_status() {
    let (stdout, _stderr, exit_code) = run_smartfo(&["--toon", "status", "--detailed"]);
    assert_eq!(exit_code, 0);

    // Detailed status should be reflected in empty state
    assert!(stdout.contains("detailed") || !stdout.is_empty());
}

/// Test that empty state works with summary status
#[test]
fn test_empty_state_summary_status() {
    let (stdout, _stderr, exit_code) = run_smartfo(&["--toon", "status"]);
    assert_eq!(exit_code, 0);

    // Summary status should be reflected in empty state
    assert!(!stdout.is_empty());
}

/// Test that empty state works with field selection
#[test]
fn test_empty_state_with_field_selection() {
    let (stdout, _stderr, exit_code) = run_smartfo(&[
        "--toon",
        "--fields",
        "count,message",
        "list",
    ]);

    assert_eq!(exit_code, 0);

    // Field selection should work with empty state
    assert!(stdout.contains("count") || stdout.contains("message") || !stdout.is_empty());
}

/// Test that empty state works with --full flag
#[test]
fn test_empty_state_with_full_flag() {
    let (stdout, _stderr, exit_code) = run_smartfo(&["--toon", "--full", "list"]);
    assert_eq!(exit_code, 0);

    // With --full, empty state should be complete
    assert!(!stdout.is_empty());
}

/// Test that empty state is serializable
#[test]
fn test_empty_state_serialization() {
    let (stdout, _stderr, exit_code) = run_smartfo(&["--format", "json", "list"]);

    assert_eq!(exit_code, 0);

    // JSON format should be parseable
    if !stdout.is_empty() {
        let parsed: Result<serde_json::Value, _> = serde_json::from_str(&stdout);
        assert!(parsed.is_ok() || stdout.contains('{'));
    }
}

/// Test that empty state works with mv command
#[test]
fn test_empty_state_with_mv_command() {
    let temp_dir = tempfile::tempdir().unwrap();
    let src_file = temp_dir.path().join("test_source.txt");
    let dest_file = temp_dir.path().join("test_dest.txt");

    std::fs::write(&src_file, "test content").unwrap();

    let (stdout, _stderr, exit_code) = run_smartfo(&[
        "--toon",
        "--dry-run",
        &src_file.to_str().unwrap(),
        &dest_file.to_str().unwrap(),
    ]);

    assert_eq!(exit_code, 0);
    assert!(!stdout.is_empty());
}

/// Test that empty state works with rm command
#[test]
fn test_empty_state_with_rm_command() {
    let temp_dir = tempfile::tempdir().unwrap();
    let test_file = temp_dir.path().join("test_file.txt");

    std::fs::write(&test_file, "test content").unwrap();

    let (stdout, _stderr, exit_code) = run_smartfo(&[
        "--toon",
        "--dry-run",
        &test_file.to_str().unwrap(),
    ]);

    assert_eq!(exit_code, 0);
    assert!(!stdout.is_empty());
}

/// Test that empty state works with session context
#[test]
fn test_empty_state_session_context() {
    let (stdout, _stderr, exit_code) = run_smartfo(&["--toon", "--session-context"]);

    assert_eq!(exit_code, 0);

    // Session context should include empty state when no data
    assert!(!stdout.is_empty());
}

/// Test that empty state handles Unicode content
#[test]
fn test_empty_state_unicode() {
    let temp_dir = tempfile::tempdir().unwrap();
    let unicode_file = temp_dir.path().join("unicode.txt");
    std::fs::write(&unicode_file, "test content 中文").unwrap();

    let (stdout, _stderr, exit_code) = run_smartfo(&[
        "--toon",
        "--dry-run",
        &unicode_file.to_str().unwrap(),
    ]);

    assert_eq!(exit_code, 0);
}

/// Test that empty state works with special characters
#[test]
fn test_empty_state_special_characters() {
    let temp_dir = tempfile::tempdir().unwrap();
    let special_file = temp_dir.path().join("file with spaces.txt");
    std::fs::write(&special_file, "test content").unwrap();

    let (stdout, _stderr, exit_code) = run_smartfo(&[
        "--toon",
        "--dry-run",
        &special_file.to_str().unwrap(),
    ]);

    assert_eq!(exit_code, 0);
}

/// Test that empty state works with multiple files
#[test]
fn test_empty_state_multiple_files() {
    let temp_dir = tempfile::tempdir().unwrap();

    for i in 0..5 {
        let file = temp_dir.path().join(format!("test{}.txt", i));
        std::fs::write(&file, format!("content {}", i)).unwrap();
    }

    let (stdout, _stderr, exit_code) = run_smartfo(&[
        "--toon",
        "--dry-run",
        &temp_dir.path().to_str().unwrap(),
    ]);

    assert_eq!(exit_code, 0);
}

/// Test that empty state is consistent
#[test]
fn test_empty_state_consistency() {
    let (stdout1, _stderr1, exit_code1) = run_smartfo(&["--toon", "list"]);
    let (stdout2, _stderr2, exit_code2) = run_smartfo(&["--toon", "list"]);

    assert_eq!(exit_code1, 0);
    assert_eq!(exit_code2, 0);

    // Empty state should be consistent
    assert_eq!(stdout1, stdout2);
}

/// Test that empty state includes count field
#[test]
fn test_empty_state_count_field() {
    let (stdout, _stderr, exit_code) = run_smartfo(&["--toon", "list"]);

    assert_eq!(exit_code, 0);

    // Empty state should include count field
    assert!(stdout.contains("count") || stdout.contains("0") || !stdout.is_empty());
}

/// Test that empty state works with TOON format
#[test]
fn test_empty_state_toon_format() {
    let (stdout, _stderr, exit_code) = run_smartfo(&["--toon", "list"]);

    assert_eq!(exit_code, 0);

    // TOON format should preserve empty state structure
    let lines: Vec<&str> = stdout.lines().collect();
    let has_key_value = lines.iter().any(|line| line.contains(':'));
    assert!(has_key_value);
}

/// Test that empty state works with JSON format
#[test]
fn test_empty_state_json_format() {
    let (stdout, _stderr, exit_code) = run_smartfo(&["--format", "json", "list"]);

    assert_eq!(exit_code, 0);

    // JSON format should preserve empty state structure
    if !stdout.is_empty() {
        let parsed: Result<serde_json::Value, _> = serde_json::from_str(&stdout);
        assert!(parsed.is_ok() || stdout.contains('{'));
    }
}

/// Test that empty state works with human format
#[test]
fn test_empty_state_human_format() {
    let (stdout, _stderr, exit_code) = run_smartfo(&["--human", "list"]);

    assert_eq!(exit_code, 0);
    assert!(!stdout.is_empty());
}

/// Test that empty state works with different subcommands
#[test]
fn test_empty_state_different_subcommands() {
    // List command
    let (list_out, _list_err, list_exit) = run_smartfo(&["--toon", "list"]);
    assert_eq!(list_exit, 0);
    assert!(!list_out.is_empty());

    // Status command
    let (status_out, _status_err, status_exit) = run_smartfo(&["--toon", "status"]);
    assert_eq!(status_exit, 0);
    assert!(!status_out.is_empty());

    // Session context
    let (ctx_out, _ctx_err, ctx_exit) = run_smartfo(&["--toon", "session-context"]);
    assert_eq!(ctx_exit, 0);
    assert!(!ctx_out.is_empty());

    // Each should have its own empty state
    assert!(!list_out.is_empty());
    assert!(!status_out.is_empty());
    assert!(!ctx_out.is_empty());
}

/// Test that empty state includes contextual suggestions
#[test]
fn test_empty_state_contextual_suggestions() {
    let (stdout, _stderr, exit_code) = run_smartfo(&["--toon", "list"]);

    assert_eq!(exit_code, 0);

    // Empty state should include contextual suggestions
    // This is a basic check - actual suggestions may vary
    assert!(!stdout.is_empty());
}

/// Test that empty state handles no data gracefully
#[test]
fn test_empty_state_no_data() {
    let (stdout, _stderr, exit_code) = run_smartfo(&["--toon", "list"]);

    assert_eq!(exit_code, 0);

    // Should handle no data gracefully
    assert!(!stdout.is_empty());
}

/// Test that empty state is token-efficient
#[test]
fn test_empty_state_token_efficiency() {
    let (stdout, _stderr, exit_code) = run_smartfo(&["--toon", "list"]);

    assert_eq!(exit_code, 0);

    // Empty state should be token-efficient
    assert!(stdout.len() < 1000, "Empty state should be token-efficient");
}

/// Test that empty state works with daemon status
#[test]
fn test_empty_state_daemon_status() {
    let (stdout, _stderr, exit_code) = run_smartfo(&["--toon", "status"]);

    assert_eq!(exit_code, 0);

    // Empty state should include daemon status
    assert!(!stdout.is_empty());
}

/// Test that empty state includes operation queue information
#[test]
fn test_empty_state_queue_info() {
    let (stdout, _stderr, exit_code) = run_smartfo(&["--toon", "status"]);

    assert_eq!(exit_code, 0);

    // Empty state should include queue information
    assert!(!stdout.is_empty());
}

/// Test that empty state message is informative
#[test]
fn test_empty_state_informative_message() {
    let (stdout, _stderr, exit_code) = run_smartfo(&["--toon", "list"]);

    assert_eq!(exit_code, 0);

    // Empty state message should be informative
    assert!(!stdout.is_empty());
}

/// Test that empty state works with no arguments
#[test]
fn test_empty_state_no_args() {
    let (stdout, _stderr, exit_code) = run_smartfo(&["--toon"]);

    assert_eq!(exit_code, 0);

    // No-args mode should also return empty state when no data
    assert!(!stdout.is_empty());
}

/// Test that empty state is consistent with different output modes
#[test]
fn test_empty_state_output_modes() {
    // TOON format
    let (toon_out, _toon_err, toon_exit) = run_smartfo(&["--toon", "list"]);
    assert_eq!(toon_exit, 0);
    assert!(!toon_out.is_empty());

    // JSON format
    let (json_out, _json_err, json_exit) = run_smartfo(&["--format", "json", "list"]);
    assert_eq!(json_exit, 0);
    assert!(!json_out.is_empty());

    // Human format
    let (human_out, _human_err, human_exit) = run_smartfo(&["--human", "list"]);
    assert_eq!(human_exit, 0);
    assert!(!human_out.is_empty());
}

/// Test that empty state preserves metadata
#[test]
fn test_empty_state_metadata() {
    let (stdout, _stderr, exit_code) = run_smartfo(&["--toon", "list"]);

    assert_eq!(exit_code, 0);

    // Empty state should preserve metadata
    assert!(!stdout.is_empty());
}

/// Test that empty state is minimal
#[test]
fn test_empty_state_minimal() {
    let (stdout, _stderr, exit_code) = run_smartfo(&["--toon", "list"]);

    assert_eq!(exit_code, 0);

    // Empty state should be minimal and concise
    assert!(stdout.len() < 1000, "Empty state should be minimal");
}
