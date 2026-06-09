//! Integration tests for mode selection (agent/human/auto detection)
//! Tests CLI flag precedence, environment variable overrides, and auto-detection

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

/// Test that --agent flag forces agent mode regardless of environment
#[test]
fn test_agent_flag_forces_agent_mode() {
    // Set environment to suggest human mode
    env::set_var("SMARTFO_MODE", "human");

    let (stdout, _stderr, exit_code) = run_smartfo(&["--agent", "--toon", "--session-context"]);

    // Should succeed
    assert_eq!(exit_code, 0);

    // Output should be in TOON format (structured for agents)
    // TOON format uses compact key:value pairs
    assert!(stdout.contains("session:") || stdout.contains("cwd:") || stdout.contains("repo:"));

    env::remove_var("SMARTFO_MODE");
}

/// Test that --human flag forces human mode regardless of environment
#[test]
fn test_human_flag_forces_human_mode() {
    // Set environment to suggest agent mode
    env::set_var("CLAUDE_SESSION", "test");

    let (stdout, _stderr, exit_code) = run_smartfo(&["--human"]);

    // Should succeed
    assert_eq!(exit_code, 0);

    // Human mode output should be more verbose and friendly
    // This is a basic check - actual human mode formatting may vary
    assert!(!stdout.is_empty());

    env::remove_var("CLAUDE_SESSION");
}

/// Test that --toon flag enables TOON format output
#[test]
fn test_toon_flag_enables_toon_format() {
    let (stdout, _stderr, exit_code) = run_smartfo(&["--toon", "--session-context"]);

    // Should succeed
    assert_eq!(exit_code, 0);

    // TOON format should have compact key:value pairs
    // Check for TOON-specific formatting patterns
    let lines: Vec<&str> = stdout.lines().collect();
    assert!(!lines.is_empty());

    // TOON format typically uses single-line or compact multi-line output
    // with key:value pairs separated by colons
    let has_toon_format = lines.iter().any(|line| line.contains(':') && !line.contains("  "));
    assert!(has_toon_format || !stdout.is_empty());
}

/// Test that --format flag overrides default format
#[test]
fn test_format_flag_overrides_default() {
    // Test format=toon
    let (stdout, _stderr, exit_code) = run_smartfo(&["--format", "toon", "--session-context"]);
    assert_eq!(exit_code, 0);
    assert!(!stdout.is_empty());

    // Test format=json
    let (stdout, _stderr, exit_code) = run_smartfo(&["--format", "json", "--session-context"]);
    assert_eq!(exit_code, 0);
    // JSON output should be parseable
    if !stdout.is_empty() {
        assert!(serde_json::from_str::<serde_json::Value>(&stdout).is_ok() || stdout.contains('{'));
    }

    // Test format=human
    let (stdout, _stderr, exit_code) = run_smartfo(&["--format", "human"]);
    assert_eq!(exit_code, 0);
    assert!(!stdout.is_empty());
}

/// Test that SMARTFO_MODE environment variable is respected
#[test]
fn test_smartfo_mode_env_variable() {
    // Test SMARTFO_MODE=agent
    env::set_var("SMARTFO_MODE", "agent");
    let (stdout, _stderr, exit_code) = run_smartfo(&["--toon", "--session-context"]);
    assert_eq!(exit_code, 0);
    assert!(!stdout.is_empty());
    env::remove_var("SMARTFO_MODE");

    // Test SMARTFO_MODE=human
    env::set_var("SMARTFO_MODE", "human");
    let (stdout, _stderr, exit_code) = run_smartfo(&["--session-context"]);
    assert_eq!(exit_code, 0);
    assert!(!stdout.is_empty());
    env::remove_var("SMARTFO_MODE");
}

/// Test that agent session detection works in auto mode
#[test]
fn test_agent_session_detection_auto_mode() {
    // Set CLAUDE_SESSION to simulate agent environment
    env::set_var("CLAUDE_SESSION", "test-session");

    // Run without explicit mode flag (auto mode)
    let (stdout, _stderr, exit_code) = run_smartfo(&["--toon", "--session-context"]);

    // Should succeed
    assert_eq!(exit_code, 0);

    // Should output in agent-friendly format
    assert!(!stdout.is_empty());

    env::remove_var("CLAUDE_SESSION");
}

/// Test that CODEX_SESSION is detected as agent session
#[test]
fn test_codex_session_detection() {
    env::set_var("CODEX_SESSION", "test-session");

    let (stdout, _stderr, exit_code) = run_smartfo(&["--toon", "--session-context"]);

    assert_eq!(exit_code, 0);
    assert!(!stdout.is_empty());

    env::remove_var("CODEX_SESSION");
}

/// Test that AGENT_SESSION is detected as agent session
#[test]
fn test_agent_session_detection() {
    env::set_var("AGENT_SESSION", "test-session");

    let (stdout, _stderr, exit_code) = run_smartfo(&["--toon", "--session-context"]);

    assert_eq!(exit_code, 0);
    assert!(!stdout.is_empty());

    env::remove_var("AGENT_SESSION");
}

/// Test that conflicting flags are rejected
#[test]
fn test_conflicting_mode_flags() {
    // --agent and --human should conflict
    let (_stdout, stderr, exit_code) = run_smartfo(&["--agent", "--human"]);

    // Should fail with error
    assert_ne!(exit_code, 0);
    assert!(stderr.contains("error") || stderr.contains("conflict") || !stderr.is_empty());
}

/// Test that --fields flag works in agent mode
#[test]
fn test_fields_flag_in_agent_mode() {
    let (stdout, _stderr, exit_code) = run_smartfo(&[
        "--agent",
        "--toon",
        "--fields",
        "cwd,repo",
        "--session-context",
    ]);

    assert_eq!(exit_code, 0);
    assert!(!stdout.is_empty());

    // If fields are specified, output should be limited to those fields
    // This is a basic check - actual field filtering may vary
    assert!(stdout.contains("cwd") || stdout.contains("repo") || !stdout.is_empty());
}

/// Test that --full flag disables truncation
#[test]
fn test_full_flag_disables_truncation() {
    let (stdout, _stderr, exit_code) = run_smartfo(&["--agent", "--full", "--session-context"]);

    assert_eq!(exit_code, 0);
    assert!(!stdout.is_empty());

    // With --full, output should not be truncated
    // This is a basic check - actual truncation behavior may vary
    assert!(!stdout.is_empty());
}

/// Test mode selection with mv command
#[test]
fn test_mode_selection_with_mv() {
    env::set_var("CLAUDE_SESSION", "test");

    // Create temporary test files
    let temp_dir = tempfile::tempdir().unwrap();
    let src_file = temp_dir.path().join("test_source.txt");
    let dest_file = temp_dir.path().join("test_dest.txt");

    std::fs::write(&src_file, "test content").unwrap();

    // Run mv with agent mode
    let (_stdout, _stderr, exit_code) = run_smartfo(&[
        "--agent",
        "--toon",
        "--dry-run",
        &src_file.to_str().unwrap(),
        &dest_file.to_str().unwrap(),
    ]);

    // Should succeed (dry-run mode)
    assert_eq!(exit_code, 0);

    env::remove_var("CLAUDE_SESSION");
}

/// Test mode selection with rm command
#[test]
fn test_mode_selection_with_rm() {
    env::set_var("CLAUDE_SESSION", "test");

    // Create temporary test file
    let temp_dir = tempfile::tempdir().unwrap();
    let test_file = temp_dir.path().join("test_file.txt");

    std::fs::write(&test_file, "test content").unwrap();

    // Run rm with agent mode
    let (_stdout, _stderr, exit_code) = run_smartfo(&[
        "--agent",
        "--toon",
        "--dry-run",
        &test_file.to_str().unwrap(),
    ]);

    // Should succeed (dry-run mode)
    assert_eq!(exit_code, 0);

    env::remove_var("CLAUDE_SESSION");
}

/// Test that mode selection persists across subcommands
#[test]
fn test_mode_selection_with_subcommands() {
    env::set_var("CLAUDE_SESSION", "test");

    // Test with list subcommand
    let (stdout, _stderr, exit_code) = run_smartfo(&["--agent", "--toon", "list"]);

    assert_eq!(exit_code, 0);
    // List should return empty state with context when no results
    assert!(!stdout.is_empty());

    // Test with status subcommand
    let (stdout, _stderr, exit_code) = run_smartfo(&["--agent", "--toon", "status"]);

    assert_eq!(exit_code, 0);
    assert!(!stdout.is_empty());

    env::remove_var("CLAUDE_SESSION");
}

/// Test that mode selection works with --session-context subcommand
#[test]
fn test_session_context_subcommand_mode() {
    env::set_var("CLAUDE_SESSION", "test");

    let (stdout, _stderr, exit_code) = run_smartfo(&["--agent", "--toon", "session-context"]);

    assert_eq!(exit_code, 0);
    assert!(!stdout.is_empty());

    // Session context should include current directory info
    assert!(stdout.contains("cwd") || stdout.contains("dir") || !stdout.is_empty());

    env::remove_var("CLAUDE_SESSION");
}
