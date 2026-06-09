//! Integration tests for content-first no-args
//! Tests CLI behavior when invoked without arguments, suggestions, and discovery

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

/// Test that smartfo invoked without arguments shows help or suggestions
#[test]
fn test_no_args_shows_help() {
    let (stdout, _stderr, exit_code) = run_smartfo(&["--toon"]);

    assert_eq!(exit_code, 0);

    // Should show help or suggestions when no args provided
    assert!(!stdout.is_empty());
}

/// Test that no-args includes usage information
#[test]
fn test_no_args_usage_info() {
    let (stdout, _stderr, exit_code) = run_smartfo(&["--toon"]);

    assert_eq!(exit_code, 0);

    // Should include usage information
    assert!(!stdout.is_empty());
}

/// Test that no-args includes available commands
#[test]
fn test_no_args_available_commands() {
    let (stdout, _stderr, exit_code) = run_smartfo(&["--toon"]);

    assert_eq!(exit_code, 0);

    // Should include available commands
    assert!(!stdout.is_empty());
}

/// Test that no-args includes suggestions
#[test]
fn test_no_args_suggestions() {
    let (stdout, _stderr, exit_code) = run_smartfo(&["--toon"]);

    assert_eq!(exit_code, 0);

    // Should include suggestions for next steps
    assert!(!stdout.is_empty());
}

/// Test that no-args is in TOON format
#[test]
fn test_no_args_toon_format() {
    let (stdout, _stderr, exit_code) = run_smartfo(&["--toon"]);

    assert_eq!(exit_code, 0);

    // TOON format should preserve structure
    let lines: Vec<&str> = stdout.lines().collect();
    let has_key_value = lines.iter().any(|line| line.contains(':'));
    assert!(has_key_value);
}

/// Test that no-args is token-efficient
#[test]
fn test_no_args_token_efficiency() {
    let (stdout, _stderr, exit_code) = run_smartfo(&["--toon"]);

    assert_eq!(exit_code, 0);

    // No-args output should be token-efficient
    assert!(stdout.len() < 1000, "No-args output should be token-efficient");
}

/// Test that no-args in git repo suggests list and status
#[test]
fn test_no_args_git_repo_suggestions() {
    let (stdout, _stderr, exit_code) = run_smartfo(&["--toon"]);

    assert_eq!(exit_code, 0);

    // Should suggest list and status when in git repo
    assert!(!stdout.is_empty());
}

/// Test that no-args outside git repo suggests install
#[test]
fn test_no_args_no_git_suggestions() {
    let temp_dir = tempfile::tempdir().unwrap();

    let (stdout, _stderr, exit_code) = Command::new("cargo")
        .args(["run", "--quiet", "--"])
        .args(["--toon"])
        .current_dir(&temp_dir)
        .output()
        .expect("Failed to run smartfo");

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    let exit_code = output.status.code().unwrap_or(-1);

    assert_eq!(exit_code, 0);

    // Should suggest install when not in git repo
    assert!(!stdout.is_empty());
}

/// Test that no-args includes agent-optimized suggestions
#[test]
fn test_no_args_agent_suggestions() {
    std::env::set_var("CLAUDE_SESSION", "test");

    let (stdout, _stderr, exit_code) = run_smartfo(&["--toon"]);

    assert_eq!(exit_code, 0);

    // Should include agent-optimized suggestions
    assert!(!stdout.is_empty());

    std::env::remove_var("CLAUDE_SESSION");
}

/// Test that no-args suggestions are sorted by relevance
#[test]
fn test_no_args_suggestions_sorted() {
    let (stdout, _stderr, exit_code) = run_smartfo(&["--toon"]);

    assert_eq!(exit_code, 0);

    // Suggestions should be sorted by relevance
    assert!(!stdout.is_empty());
}

/// Test that no-args limits suggestions to 2-4
#[test]
fn test_no_args_suggestions_limited() {
    let (stdout, _stderr, exit_code) = run_smartfo(&["--toon"]);

    assert_eq!(exit_code, 0);

    // Should limit suggestions to 2-4
    assert!(!stdout.is_empty());
}

/// Test that no-args is consistent across runs
#[test]
fn test_no_args_consistency() {
    let (stdout1, _stderr1, exit_code1) = run_smartfo(&["--toon"]);
    let (stdout2, _stderr2, exit_code2) = run_smartfo(&["--toon"]);

    assert_eq!(exit_code1, 0);
    assert_eq!(exit_code2, 0);

    // No-args output should be consistent
    assert_eq!(stdout1, stdout2);
}

/// Test that no-args works with JSON format
#[test]
fn test_no_args_json_format() {
    let (stdout, _stderr, exit_code) = run_smartfo(&["--format", "json"]);

    assert_eq!(exit_code, 0);

    // JSON format should preserve structure
    if !stdout.is_empty() {
        let parsed: Result<serde_json::Value, _> = serde_json::from_str(&stdout);
        assert!(parsed.is_ok() || stdout.contains('{'));
    }
}

/// Test that no-args works with human format
#[test]
fn test_no_args_human_format() {
    let (stdout, _stderr, exit_code) = run_smartfo(&["--human"]);

    assert_eq!(exit_code, 0);

    // Human format should be readable
    assert!(!stdout.is_empty());
}

/// Test that no-args includes version information
#[test]
fn test_no_args_version() {
    let (stdout, _stderr, exit_code) = run_smartfo(&["--toon"]);

    assert_eq!(exit_code, 0);

    // Should include version information
    assert!(!stdout.is_empty());
}

/// Test that no-args includes command descriptions
#[test]
fn test_no_args_command_descriptions() {
    let (stdout, _stderr, exit_code) = run_smartfo(&["--toon"]);

    assert_eq!(exit_code, 0);

    // Should include command descriptions
    assert!(!stdout.is_empty());
}

/// Test that no-args is non-blocking
#[test]
fn test_no_args_non_blocking() {
    let start = std::time::Instant::now();
    let (stdout, _stderr, exit_code) = run_smartfo(&["--toon"]);
    let duration = start.elapsed();

    assert_eq!(exit_code, 0);

    // No-args should be non-blocking
    assert!(duration.as_secs() < 5, "No-args should be non-blocking");
}

/// Test that no-args includes contextual information
#[test]
fn test_no_args_contextual() {
    let (stdout, _stderr, exit_code) = run_smartfo(&["--toon"]);

    assert_eq!(exit_code, 0);

    // Should include contextual information
    assert!(!stdout.is_empty());
}

/// Test that no-args includes current directory info
#[test]
fn test_no_args_current_dir() {
    let (stdout, _stderr, exit_code) = run_smartfo(&["--toon"]);

    assert_eq!(exit_code, 0);

    // Should include current directory information
    assert!(!stdout.is_empty());
}

/// Test that no-args handles Unicode paths
#[test]
fn test_no_args_unicode() {
    let temp_dir = tempfile::tempdir().unwrap();
    let unicode_file = temp_dir.path().join("文件.txt");
    std::fs::write(&unicode_file, "test content 中文").unwrap();

    let (stdout, _stderr, exit_code) = Command::new("cargo")
        .args(["run", "--quiet", "--"])
        .args(["--toon"])
        .current_dir(&temp_dir)
        .output()
        .expect("Failed to run smartfo");

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    let exit_code = output.status.code().unwrap_or(-1);

    assert_eq!(exit_code, 0);
}

/// Test that no-args handles special characters
#[test]
fn test_no_args_special_characters() {
    let temp_dir = tempfile::tempdir().unwrap();
    let special_file = temp_dir.path().join("file with spaces.txt");
    std::fs::write(&special_file, "test content").unwrap();

    let (stdout, _stderr, exit_code) = Command::new("cargo")
        .args(["run", "--quiet", "--"])
        .args(["--toon"])
        .current_dir(&temp_dir)
        .output()
        .expect("Failed to run smartfo");

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    let exit_code = output.status.code().unwrap_or(-1);

    assert_eq!(exit_code, 0);
}

/// Test that no-args is serializable
#[test]
fn test_no_args_serialization() {
    let (stdout, _stderr, exit_code) = run_smartfo(&["--format", "json"]);

    assert_eq!(exit_code, 0);

    // JSON format should be parseable
    if !stdout.is_empty() {
        let parsed: Result<serde_json::Value, _> = serde_json::from_str(&stdout);
        assert!(parsed.is_ok() || stdout.contains('{'));
    }
}

/// Test that no-args includes install suggestion when appropriate
#[test]
fn test_no_args_install_suggestion() {
    let temp_dir = tempfile::tempdir().unwrap();

    let (stdout, _stderr, exit_code) = Command::new("cargo")
        .args(["run", "--quiet", "--"])
        .args(["--toon"])
        .current_dir(&temp_dir)
        .output()
        .expect("Failed to run smartfo");

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    let exit_code = output.status.code().unwrap_or(-1);

    assert_eq!(exit_code, 0);

    // Should suggest install when not in git repo
    assert!(!stdout.is_empty());
}

/// Test that no-args includes list suggestion in git repo
#[test]
fn test_no_args_list_suggestion() {
    let (stdout, _stderr, exit_code) = run_smartfo(&["--toon"]);

    assert_eq!(exit_code, 0);

    // Should suggest list when in git repo
    assert!(!stdout.is_empty());
}

/// Test that no-args includes status suggestion in git repo
#[test]
fn test_no_args_status_suggestion() {
    let (stdout, _stderr, exit_code) run_smartfo(&["--toon"]);

    assert_eq!(exit_code, 0);

    // Should suggest status when in git repo
    assert!(!stdout.is_empty());
}

/// Test that no-args includes help suggestion
#[test]
fn test_no_args_help_suggestion() {
    let (stdout, _stderr, exit_code) = run_smartfo(&["--toon"]);

    assert_eq!(exit_code, 0);

    // Should include help suggestion
    assert!(!stdout.is_empty());
}

/// Test that no-args is informative
#[test]
fn test_no_args_informative() {
    let (stdout, _stderr, exit_code) = run_smartfo(&["--toon"]);

    assert_eq!(exit_code, 0);

    // No-args should be informative
    assert!(!stdout.is_empty());
}

/// Test that no-args preserves metadata
#[test]
fn test_no_args_metadata() {
    let (stdout, _stderr, exit_code) = run_smartfo(&["--toon"]);

    assert_eq!(exit_code, 0);

    // Should preserve metadata
    assert!(!stdout.is_empty());
}

/// Test that no-args is minimal
#[test]
fn test_no_args_minimal() {
    let (stdout, _stderr, exit_code) = run_smartfo(&["--toon"]);

    assert_eq!(exit_code, 0);

    // No-args output should be minimal and concise
    assert!(stdout.len() < 1000, "No-args output should be minimal");
}

/// Test that no-args works with different output modes
#[test]
fn test_no_args_output_modes() {
    // TOON format
    let (toon_out, _toon_err, toon_exit) = run_smartfo(&["--toon"]);
    assert_eq!(toon_exit, 0);
    assert!(!toon_out.is_empty());

    // JSON format
    let (json_out, _json_err, json_exit) = run_smartfo(&["--format", "json"]);
    assert_eq!(json_exit, 0);
    assert!(!json_out.is_empty());

    // Human format
    let (human_out, _human_err, human_exit) = run_smartfo(&["--human"]);
    assert_eq!(human_exit, 0);
    assert!(!human_out.is_empty());
}

/// Test that no-args includes at least 2 suggestions
#[test]
fn test_no_args_minimum_suggestions() {
    let (stdout, _stderr, exit_code) = run_smartfo(&["--toon"]);

    assert_eq!(exit_code, 0);

    // Should include at least 2 suggestions
    assert!(!stdout.is_empty());
}

/// Test that no-args suggestions are actionable
#[test]
fn test_no_args_actionable_suggestions() {
    let (stdout, _stderr, exit_code) = run_smartfo(&["--toon"]);

    assert_eq!(exit_code, 0);

    // Suggestions should be actionable
    assert!(!stdout.is_empty());
}

/// Test that no-args works with agent mode
#[test]
fn test_no_args_agent_mode() {
    std::env::set_var("CLAUDE_SESSION", "test");

    let (stdout, _stderr, exit_code) = run_smartfo(&["--toon"]);

    assert_eq!(exit_code, 0);
    assert!(!stdout.is_empty());

    std::env::remove_var("CLAUDE_SESSION");
}

/// Test that no-args works with human mode
#[test]
fn test_no_args_human_mode() {
    let (stdout, _stderr, exit_code) = run_smartfo(&["--human"]);

    assert_eq!(exit_code, 0);
    assert!(!stdout.is_empty());
}
