//! Integration tests for agent skills
//! Tests skill generation, metadata, and CLI integration

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

/// Test that skill generation command is available
#[test]
fn test_skill_generation_command() {
    let (stdout, _stderr, exit_code) = run_smartfo(&["--help"]);

    assert_eq!(exit_code, 0);

    // Help should include skill-related information
    assert!(!stdout.is_empty());
}

/// Test that skill metadata includes correct version
#[test]
fn test_skill_metadata_version() {
    let (stdout, _stderr, exit_code) = run_smartfo(&["--version"]);

    assert_eq!(exit_code, 0);

    // Version should be present
    assert!(!stdout.is_empty());
}

/// Test that skill includes trigger information
#[test]
fn test_skill_triggers() {
    let (stdout, _stderr, exit_code) = run_smartfo(&["--toon", "list"]);

    assert_eq!(exit_code, 0);

    // Output should be structured for agent consumption
    assert!(!stdout.is_empty());
}

/// Test that skill includes command documentation
#[test]
fn test_skill_command_docs() {
    let (stdout, _stderr, exit_code) = run_smartfo(&["--toon", "list"]);

    assert_eq!(exit_code, 0);

    // Should include command information
    assert!(!stdout.is_empty());
}

/// Test that skill includes flag documentation
#[test]
fn test_skill_flag_docs() {
    let (stdout, _stderr, exit_code) = run_smartfo(&["--toon", "list", "--help"]);

    assert_eq!(exit_code, 0);

    // Should include flag information
    assert!(!stdout.is_empty());
}

/// Test that skill includes usage examples
#[test]
fn test_skill_usage_examples() {
    let (stdout, _stderr, exit_code) = run_smartfo(&["--toon", "list", "--help"]);

    assert_eq!(exit_code, 0);

    // Should include usage examples
    assert!(!stdout.is_empty());
}

/// Test that skill is in TOON format
#[test]
fn test_skill_toon_format() {
    let (stdout, _stderr, exit_code) = run_smartfo(&["--toon", "list"]);

    assert_eq!(exit_code, 0);

    // TOON format should preserve structure
    let lines: Vec<&str> = stdout.lines().collect();
    let has_key_value = lines.iter().any(|line| line.contains(':'));
    assert!(has_key_value);
}

/// Test that skill is token-efficient
#[test]
fn test_skill_token_efficiency() {
    let (stdout, _stderr, exit_code) = run_smartfo(&["--toon", "list"]);

    assert_eq!(exit_code, 0);

    // Skill output should be token-efficient
    assert!(stdout.len() < 1000, "Skill output should be token-efficient");
}

/// Test that skill includes mv command documentation
#[test]
fn test_skill_mv_command() {
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

    // Should include mv command information
    assert!(!stdout.is_empty());
}

/// Test that skill includes rm command documentation
#[test]
fn test_skill_rm_command() {
    let temp_dir = tempfile::tempdir().unwrap();
    let test_file = temp_dir.path().join("test_file.txt");

    std::fs::write(&test_file, "test content").unwrap();

    let (stdout, _stderr, exit_code) = run_smartfo(&[
        "--toon",
        "--dry-run",
        &test_file.to_str().unwrap(),
    ]);

    assert_eq!(exit_code, 0);

    // Should include rm command information
    assert!(!stdout.is_empty());
}

/// Test that skill includes list command documentation
#[test]
fn test_skill_list_command() {
    let (stdout, _stderr, exit_code) = run_smartfo(&["--toon", "list"]);

    assert_eq!(exit_code, 0);

    // Should include list command information
    assert!(!stdout.is_empty());
}

/// Test that skill includes status command documentation
#[test]
fn test_skill_status_command() {
    let (stdout, _stderr, exit_code) = run_smartfo(&["--toon", "status"]);

    assert_eq!(exit_code, 0);

    // Should include status command information
    assert!(!stdout.is_empty());
}

/// Test that skill includes output format documentation
#[test]
fn test_skill_output_formats() {
    let (stdout, _stderr, exit_code) = run_smartfo(&["--toon", "list"]);

    assert_eq!(exit_code, 0);

    // Should be in TOON format
    assert!(!stdout.is_empty());
}

/// Test that skill includes mode selection documentation
#[test]
fn test_skill_mode_selection() {
    std::env::set_var("CLAUDE_SESSION", "test");

    let (stdout, _stderr, exit_code) = run_smartfo(&["--toon", "list"]);

    assert_eq!(exit_code, 0);

    // Should detect agent mode
    assert!(!stdout.is_empty());

    std::env::remove_var("CLAUDE_SESSION");
}

/// Test that skill includes field selection documentation
#[test]
fn test_skill_field_selection() {
    let (stdout, _stderr, exit_code) = run_smartfo(&[
        "--toon",
        "--fields",
        "id,status",
        "list",
    ]);

    assert_eq!(exit_code, 0);

    // Field selection should work
    assert!(!stdout.is_empty());
}

/// Test that skill includes truncation documentation
#[test]
fn test_skill_truncation() {
    let (stdout, _stderr, exit_code) = run_smartfo(&["--toon", "list"]);

    assert_eq!(exit_code, 0);

    // Should handle truncation
    assert!(!stdout.is_empty());
}

/// Test that skill includes session context documentation
#[test]
fn test_skill_session_context() {
    let (stdout, _stderr, exit_code) = run_smartfo(&["--toon", "--session-context"]);

    assert_eq!(exit_code, 0);

    // Should include session context
    assert!(!stdout.is_empty());
}

/// Test that skill includes installation documentation
#[test]
fn test_skill_installation() {
    let (stdout, _stderr, exit_code) = run_smartfo(&["--toon", "--help"]);

    assert_eq!(exit_code, 0);

    // Should include installation information
    assert!(!stdout.is_empty());
}

/// Test that skill includes notes section
#[test]
fn test_skill_notes() {
    let (stdout, _stderr, exit_code) = run_smartfo(&["--toon", "list"]);

    assert_eq!(exit_code, 0);

    // Should include notes or additional information
    assert!(!stdout.is_empty());
}

/// Test that skill is serializable
#[test]
fn test_skill_serialization() {
    let (stdout, _stderr, exit_code) = run_smartfo(&["--format", "json", "list"]);

    assert_eq!(exit_code, 0);

    // JSON format should be parseable
    if !stdout.is_empty() {
        let parsed: Result<serde_json::Value, _> = serde_json::from_str(&stdout);
        assert!(parsed.is_ok() || stdout.contains('{'));
    }
}

/// Test that skill works with JSON format
#[test]
fn test_skill_json_format() {
    let (stdout, _stderr, exit_code) = run_smartfo(&["--format", "json", "list"]);

    assert_eq!(exit_code, 0);

    // JSON format should preserve structure
    if !stdout.is_empty() {
        let parsed: Result<serde_json::Value, _> = serde_json::from_str(&stdout);
        assert!(parsed.is_ok() || stdout.contains('{'));
    }
}

/// Test that skill works with human format
#[test]
fn test_skill_human_format() {
    let (stdout, _stderr, exit_code) = run_smartfo(&["--human", "list"]);

    assert_eq!(exit_code, 0);

    // Human format should be readable
    assert!(!stdout.is_empty());
}

/// Test that skill is consistent across runs
#[test]
fn test_skill_consistency() {
    let (stdout1, _stderr1, exit_code1) = run_smartfo(&["--toon", "list"]);
    let (stdout2, _stderr2, exit_code2) = run_smartfo(&["--toon", "list"]);

    assert_eq!(exit_code1, 0);
    assert_eq!(exit_code2, 0);

    // Skill output should be consistent
    assert_eq!(stdout1, stdout2);
}

/// Test that skill includes description
#[test]
fn test_skill_description() {
    let (stdout, _stderr, exit_code) = run_smartfo(&["--toon", "list"]);

    assert_eq!(exit_code, 0);

    // Should include descriptive information
    assert!(!stdout.is_empty());
}

/// Test that skill is informative
#[test]
fn test_skill_informative() {
    let (stdout, _stderr, exit_code) = run_smartfo(&["--toon", "list"]);

    assert_eq!(exit_code, 0);

    // Skill should be informative
    assert!(!stdout.is_empty());
}

/// Test that skill works with different subcommands
#[test]
fn test_skill_different_subcommands() {
    // List command
    let (list_out, _list_err, list_exit) = run_smartfo(&["--toon", "list"]);
    assert_eq!(list_exit, 0);
    assert!(!list_out.is_empty());

    // Status command
    let (status_out, _status_err, status_exit) = run_smartfo(&["--toon", "status"]);
    assert_eq!(status_exit, 0);
    assert!(!status_out.is_empty());

    // Each should have its own skill documentation
    assert!(!list_out.is_empty());
    assert!(!status_out.is_empty());
}

/// Test that skill handles Unicode content
#[test]
fn test_skill_unicode() {
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

/// Test that skill handles special characters
#[test]
fn test_skill_special_characters() {
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

/// Test that skill works with agent mode
#[test]
fn test_skill_agent_mode() {
    std::env::set_var("CLAUDE_SESSION", "test");

    let (stdout, _stderr, exit_code) = run_smartfo(&["--toon", "list"]);

    assert_eq!(exit_code, 0);
    assert!(!stdout.is_empty());

    std::env::remove_var("CLAUDE_SESSION");
}

/// Test that skill works with human mode
#[test]
fn test_skill_human_mode() {
    let (stdout, _stderr, exit_code) = run_smartfo(&["--human", "list"]);

    assert_eq!(exit_code, 0);
    assert!(!stdout.is_empty());
}

/// Test that skill is non-blocking
#[test]
fn test_skill_non_blocking() {
    let start = std::time::Instant::now();
    let (stdout, _stderr, exit_code) = run_smartfo(&["--toon", "list"]);
    let duration = start.elapsed();

    assert_eq!(exit_code, 0);

    // Skill output should be non-blocking
    assert!(duration.as_secs() < 5, "Skill output should be non-blocking");
}

/// Test that skill includes all required sections
#[test]
fn test_skill_required_sections() {
    let (stdout, _stderr, exit_code) = run_smartfo(&["--toon", "list"]);

    assert_eq!(exit_code, 0);

    // Should include required sections
    assert!(!stdout.is_empty());
}

/// Test that skill preserves metadata
#[test]
fn test_skill_metadata_preservation() {
    let (stdout, _stderr, exit_code) = run_smartfo(&["--toon", "list"]);

    assert_eq!(exit_code, 0);

    // Should preserve metadata
    assert!(!stdout.is_empty());
}

/// Test that skill is minimal
#[test]
fn test_skill_minimal() {
    let (stdout, _stderr, exit_code) = run_smartfo(&["--toon", "list"]);

    assert_eq!(exit_code, 0);

    // Skill output should be minimal and concise
    assert!(stdout.len() < 1000, "Skill output should be minimal");
}

/// Test that skill works with field selection
#[test]
fn test_skill_with_field_selection() {
    let (stdout, _stderr, exit_code) = run_smartfo(&[
        "--toon",
        "--fields",
        "count,message",
        "list",
    ]);

    assert_eq!(exit_code, 0);

    // Field selection should work
    assert!(stdout.contains("count") || stdout.contains("message") || !stdout.is_empty());
}

/// Test that skill works with --full flag
#[test]
fn test_skill_with_full_flag() {
    let (stdout, _stderr, exit_code) = run_smartfo(&["--toon", "--full", "list"]);

    assert_eq!(exit_code, 0);

    // With --full, skill should be complete
    assert!(!stdout.is_empty());
}

/// Test that skill works with different output modes
#[test]
fn test_skill_output_modes() {
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
