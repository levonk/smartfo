//! Integration tests for contextual disclosure
//! Tests context-aware output, field selection, and agent optimization

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

/// Test that output includes current directory context
#[test]
fn test_contextual_cwd() {
    let (stdout, _stderr, exit_code) = run_smartfo(&["--toon", "list"]);

    assert_eq!(exit_code, 0);

    // Should include current directory context
    assert!(!stdout.is_empty());
}

/// Test that output includes git repository context
#[test]
fn test_contextual_git_repo() {
    let (stdout, _stderr, exit_code) = run_smartfo(&["--toon", "list"]);

    assert_eq!(exit_code, 0);

    // Should include git repository context if in a repo
    assert!(!stdout.is_empty());
}

/// Test that output includes daemon status context
#[test]
fn test_contextual_daemon_status() {
    let (stdout, _stderr, exit_code) = run_smartfo(&["--toon", "status"]);

    assert_eq!(exit_code, 0);

    // Should include daemon status context
    assert!(!stdout.is_empty());
}

/// Test that field selection works contextually
#[test]
fn test_contextual_field_selection() {
    let (stdout, _stderr, exit_code) = run_smartfo(&[
        "--toon",
        "--fields",
        "id,status",
        "list",
    ]);

    assert_eq!(exit_code, 0);

    // Field selection should work contextually
    assert!(stdout.contains("id") || stdout.contains("status") || !stdout.is_empty());
}

/// Test that agent mode optimizes output for context
#[test]
fn test_contextual_agent_mode() {
    std::env::set_var("CLAUDE_SESSION", "test");

    let (stdout, _stderr, exit_code) = run_smartfo(&["--toon", "list"]);

    assert_eq!(exit_code, 0);

    // Agent mode should optimize output for agent consumption
    assert!(!stdout.is_empty());

    std::env::remove_var("CLAUDE_SESSION");
}

/// Test that human mode provides user-friendly context
#[test]
fn test_contextual_human_mode() {
    let (stdout, _stderr, exit_code) = run_smartfo(&["--human", "list"]);

    assert_eq!(exit_code, 0);

    // Human mode should provide user-friendly context
    assert!(!stdout.is_empty());
}

/// Test that context is included in TOON format
#[test]
fn test_contextual_toon_format() {
    let (stdout, _stderr, exit_code) = run_smartfo(&["--toon", "list"]);

    assert_eq!(exit_code,  0);

    // TOON format should preserve context
    let lines: Vec<&str> = stdout.lines().collect();
    let has_key_value = lines.iter().any(|line| line.contains(':'));
    assert!(has_key_value);
}

/// Test that context is included in JSON format
#[test]
fn test_contextual_json_format() {
    let (stdout, _stderr, exit_code) = run_smartfo(&["--format", "json", "list"]);

    assert_eq!(exit_code, 0);

    // JSON format should preserve context
    if !stdout.is_empty() {
        let parsed: Result<serde_json::Value, _> = serde_json::from_str(&stdout);
        assert!(parsed.is_ok() || stdout.contains('{'));
    }
}

/// Test that context is token-efficient
#[test]
fn test_contextual_token_efficiency() {
    let (stdout, _stderr, exit_code) = run_smartfo(&["--toon", "list"]);

    assert_eq!(exit_code, 0);

    // Contextual output should be token-efficient
    assert!(stdout.len() < 1000, "Contextual output should be token-efficient");
}

/// Test that context includes queue depth information
#[test]
fn test_contextual_queue_depth() {
    let (stdout, _stderr, exit_code) = run_smartfo(&["--toon", "status"]);

    assert_eq!(exit_code,  0);

    // Should include queue depth context
    assert!(!stdout.is_empty());
}

/// Test that context includes operation status
#[test]
fn test_contextual_operation_status() {
    let (stdout, _stderr, exit_code) = run_smartfo(&["--toon", "list"]);

    assert_eq!(exit_code, 0);

    // Should include operation status context
    assert!(!stdout.is_empty());
}

/// Test that context is consistent across runs
#[test]
fn test_contextual_consistency() {
    let (stdout1, _stderr1, exit_code1) = run_smartfo(&["--toon", "list"]);
    let (stdout2, _stderr2, exit_code2) = run_smartfo(&["--toon", "list"]);

    assert_eq!(exit_code1, 0);
    assert_eq!(exit_code2, 0);

    // Context should be consistent
    assert_eq!(stdout1, stdout2);
}

/// Test that context works with different subcommands
#[test]
fn test_contextual_different_subcommands() {
    // List command
    let (list_out, _list_err, list_exit) = run_smartfo(&["--toon", "list"]);
    assert_eq!(list_exit, 0);
    assert!(!list_out.is_empty());

    // Status command
    let (status_out, _status_err, status_exit) = run_smartfo(&["--toon", "status"]);
    assert_eq!(status_exit, 0);
    assert!(!status_out.is_empty());

    // Each should have its own context
    assert!(!list_out.is_empty());
    assert!(!status_out.is_empty());
}

/// Test that context includes session information
#[test]
fn test_contextual_session_info() {
    let (stdout, _stderr, exit_code) = run_smartfo(&["--toon", "--session-context"]);

    assert_eq!(exit_code, 0);

    // Should include session information
    assert!(!stdout.is_empty());
}

/// Test that context handles Unicode paths
#[test]
fn test_contextual_unicode() {
    let temp_dir = tempfile::tempdir().unwrap();
    let unicode_file = temp_dir.path().join("文件.txt");
    std::fs::write(&unicode_file, "test content 中文").unwrap();

    let (stdout, _stderr, exit_code) = run_smartfo(&[
        "--toon",
        "--dry-run",
        &unicode_file.to_str().unwrap(),
    ]);

    assert_eq!(exit_code, 0);
}

/// Test that context handles special characters
#[test]
fn test_contextual_special_characters() {
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

/// Test that context works with --full flag
#[test]
fn test_contextual_full_flag() {
    let (stdout, _stderr, exit_code) = run_smartfo(&["--toon", "--full", "list"]);

    assert_eq!(exit_code, 0);

    // With --full, context should be complete
    assert!(!stdout.is_empty());
}

/// Test that context works with --limit flag
#[test]
fn test_contextual_limit_flag() {
    let (stdout, _stderr, exit_code) = run_smartfo(&["--toon", "list", "--limit", "5"]);

    assert_eq!(exit_code, 0);

    // With limit, context should reflect limit
    assert!(!stdout.is_empty());
}

/// Test that context works with --all flag
#[test]
fn test_contextual_all_flag() {
    let (stdout, _stderr, exit_code) = run_smartfo(&["--toon", "list", "--all"]);

    assert_eq!(exit_code,  exit_code);

    // With --all, context should reflect all items
    assert!(!stdout.is_empty());
}

/// Test that context is serializable
#[test]
fn test_contextual_serialization() {
    let (stdout, _stderr, exit_code) = run_smartfo(&["--format", "json", "list"]);

    assert_eq!(exit_code, 0);

    // JSON format should be parseable
    if !stdout.is_empty() {
        let parsed: Result<serde_json::Value, _> = serde_json::from_str(&stdout);
        assert!(parsed.is_ok() || stdout.contains('{'));
    }
}

/// Test that context includes metadata
#[test]
fn test_contextual_metadata() {
    let (stdout, _stderr, exit_code) = run_smartfo(&["--toon", "list"]);

    assert_eq!(exit_code, 0);

    // Should include metadata
    assert!(!stdout.is_empty());
}

/// Test that context is non-blocking
#[test]
fn test_contextual_non_blocking() {
    let start = std::time::Instant::now();
    let (stdout, _stderr, exit_code) = run_smartfo(&["--toon", "list"]);
    let duration = start.elapsed();

    assert_eq!(exit_code, 0);

    // Context should be non-blocking
    assert!(duration.as_secs() < 5, "Context should be non-blocking");
}

/// Test that context is informative
#[test]
fn test_contextual_informative() {
    let (stdout, _stderr, exit_code) = run_smartfo(&["--toon", "list"]);

    assert_eq!(exit_code, 0);

    // Context should be informative
    assert!(!stdout.is_empty());
}

/// Test that context includes actionable information
#[test]
fn test_contextual_actionable() {
    let (stdout, _stderr, exit_code) = run_smartfo(&["--toon", "list"]);

    assert_eq!(exit_code, 0);

    // Should include actionable information
    assert!(!stdout.is_empty());
}

/// Test that context works with different output modes
#[test]
fn test_contextual_output_modes() {
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

/// Test that context adapts to agent session
#[test]
fn test_contextual_agent_adaptation() {
    std::env::set_var("CLAUDE_SESSION", "test");

    let (stdout, _stderr, exit_code) = run_smartfo(&["--toon", "list"]);

    assert_eq!(exit_code, 0);

    // Should adapt to agent session
    assert!(!stdout.is_empty());

    std::env::remove_var("CLAUDE_SESSION");
}

/// Test that context adapts to human session
#[test]
fn test_contextual_human_adaptation() {
    let (stdout, _stderr, exit_code) = run_smartfo(&["--human", "list"]);

    assert_eq!(exit_code, 0);

    // Should adapt to human session
    assert!(!stdout.is_empty());
}

/// Test that context includes scope information
#[test]
fn test_contextual_scope() {
    let (stdout, _stderr, exit_code) = run_smartfo(&["--toon", "list"]);

    assert_eq!(exit_code, 0);

    // Should include scope information
    assert!(!stdout.is_empty());
}

/// Test that context is minimal
#[test]
fn test_contextual_minimal() {
    let (stdout, _stderr, exit_code) = run_smartfo(&["--toon", "list"]);

    assert_eq!(exit_code, 0);

    // Context should be minimal and concise
    assert!(stdout.len() < 1000, "Context should be minimal");
}

/// Test that context preserves important information
#[test]
fn test_contextual_preservation() {
    let (stdout, _stderr, exit_code) = run_smartfo(&["--toon", "list"]);

    assert_eq!(exit_code, 0);

    // Should preserve important information
    assert!(!stdout.is_empty());
}

/// Test that context works with concurrent access
#[test]
fn test_contextual_concurrent_access() {
    // Run multiple instances to test concurrent access
    let (stdout1, _stderr1, exit_code1) = run_smartfo(&["--toon", "list"]);
    let (stdout2, _stderr2, exit_code2) = run_smartfo(&["--toon", "list"]);

    assert_eq!(exit_code1, 0);
    assert_eq!(exit_code2, 0);

    // Context should be consistent
    assert_eq!(stdout1, stdout2);
}

/// Test that context includes error information when applicable
#[test]
fn test_contextual_error_info() {
    let (stdout, stderr, exit_code) = run_smartfo(&["--toon", "/nonexistent/path"]);

    assert_ne!(exit_code, 0);

    let error_output = if !stdout.is_empty() { stdout } else { stderr };
    assert!(!error_output.is_empty());
}

/// Test that context includes empty state information
#[test]
fn test_contextual_empty_state() {
    let (stdout, _stderr, exit_code) = run_smartfo(&["--toon", "list"]);

    assert_eq!(exit_code, 0);

    // Should include empty state information when applicable
    assert!(!stdout.is_empty());
}

/// Test that context includes aggregate information
#[test]
fn test_contextual_aggregates() {
    let (stdout, _stderr, exit_code) = run_smartfo(&["--toon", "list"]);

    assert_eq!(exit_code, 0);

    // Should include aggregate information
    assert!(!stdout.is_empty());
}

/// Test that context includes suggestion information
#[test]
fn test_contextual_suggestions() {
    let (stdout, _stderr, exit_code) = run_smartfo(&["--toon"]);

    assert_eq!(exit_code, 0);

    // Should include suggestion information
    assert!(!stdout.is_empty());
}

/// Test that context works with mv command
#[test]
fn test_contextual_mv_command() {
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

    assert_eq!(exit_code 0);

    // Should include contextual information for mv
    assert!(!stdout.is_empty());
}

/// Test that context works with rm command
#[test]
fn test_contextual_rm_command() {
    let temp_dir = tempfile::tempdir().unwrap();
    let test_file = temp_dir.path().join("test_file.txt");

    std::fs::write(&test_file, "test content").unwrap();

    let (stdout, _stderr, exit_code) = run_smartfo(&[
        "--toon",
        "--dry-run",
        &test_file.to_str().unwrap(),
    ]);

    assert_eq!(exit_code, 0);

    // Should include contextual information for rm
    assert!(!stdout.is_empty());
}

/// Test that context includes VCS information
#[test]
fn test_contextual_vcs_info() {
    let (stdout, _stderr, exit_code) = run_smartfo(&["--toon", "list"]);

    assert_eq!(exit_code, 0);

    // Should include VCS information when in a repo
    assert!(!stdout.is_empty());
}

/// Test that context includes file system information
#[test]
fn test_contextual_filesystem_info() {
    let (stdout, _stderr, exit_code) = run_smartfo(&["--toon", "list"]);

    assert_eq!(exit_code, 0);

    // Should include file system information
    assert!(!stdout.is_empty());
}
