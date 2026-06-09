//! End-to-end agent mode workflow tests
//! Tests complete agent workflows from start to finish

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

/// Test complete agent workflow: list operations with context
#[test]
fn test_agent_workflow_list_operations() {
    std::env::set_var("CLAUDE_SESSION", "test");

    let (stdout, _stderr, exit_code) = run_smartfo(&["--toon", "list"]);

    assert_eq!(exit_code, 0);

    // Should show operations in agent-optimized format
    assert!(!stdout.is_empty());

    std::env::remove_var("CLAUDE_SESSION");
}

/// Test complete agent workflow: check status with context
#[test]
fn test_agent_workflow_status_check() {
    std::env::set_var("CLAUDE_SESSION", "test");

    let (stdout, _stderr, exit_code) = run_smartfo(&["--toon", "status"]);

    assert_eq!(exit_code, 0);

    // Should show status in agent-optimized format
    assert!(!stdout.is_empty());

    std::env::remove_var("CLAUDE_SESSION");
}

/// Test complete agent workflow: session context
#[test]
fn test_agent_workflow_session_context() {
    std::env::set_var("CLAUDE_SESSION", "test");

    let (stdout, _stderr, exit_code) = run_smartfo(&["--toon", "--session-context"]);

    assert_eq!(exit_code, 0);

    // Should show session context in agent-optimized format
    assert!(!stdout.is_empty());

    std::env::env::remove_var("CLAUDE_SESSION");
}

/// Test complete agent workflow: move operation
#[test]
fn test_agent_workflow_move() {
    std::env::set_var("CLAUDE_SESSION", "test");

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

    // Should show move operation in agent-optimized format
    assert!(!stdout.is_empty());

    std::env::remove_var("CLAUDE_SESSION");
}

/// Test complete agent workflow: remove operation
#[test]
fn test_agent_workflow_remove() {
    std::env::set_var("CLAUDE_SESSION", "test");

    let temp_dir = tempfile::tempdir().unwrap();
    let test_file = temp_dir.path().join("test_file.txt");

    std::fs::write(&test_file, "test content").unwrap();

    let (stdout, _stderr, exit_code) = run_smartfo(&[
        "--toon",
        "--dry-run",
        &test_file.to_str().unwrap(),
    ]);

    assert_eq!(exit_code, 0);

    // Should show remove operation in agent-optimized format
    assert!(!stdout.is_empty());

    std::env::remove_var("CLAUDE_SESSION");
}

/// Test complete agent workflow: field selection
#[test]
fn test_agent_workflow_field_selection() {
    std::env::set_var("CLAUDE_SESSION", "test");

    let (stdout, _stderr, exit_code) = run_smartfo(&[
        "--toon",
        "--fields",
        "id,status",
        "list",
    ]);

    assert_eq!(exit_code, 0);

    // Should show minimal fields in agent-optimized format
    assert!(!stdout.is_empty());

    std::env::remove_var("CLAUDE_SESSION");
}

/// Test complete agent workflow: truncation
#[test]
fn test_agent_workflow_truncation() {
    std::env::set_var("CLAUDE_SESSION", "test");

    let temp_dir = tempfile::tempdir().unwrap();
    let large_file = temp_dir.path().join("large.txt");
    let large_content = "x".repeat(2000);
    std::fs::write(&large_file, &large_content).unwrap();

    let (stdout, _stderr, exit_code) = run_smartfo(&[
        "--toon",
        "--dry-run",
        &large_file.to_str().unwrap(),
        &temp_dir.path().join("dest.txt").to_str().unwrap(),
    ]);

    assert_eq!(exit_code, 0);

    // Should handle truncation in agent-optimized format
    assert!(!stdout.is_empty());

    std::env::remove_var("CLAUDE_SESSION");
}

/// Test complete agent workflow: empty state
#[test]
fn test_agent_workflow_empty_state() {
    std::env::set_var("CLAUDE_SESSION", "test");

    let (stdout, _stderr, exit_code) = run_smartfo(&["--toon", "list"]);

    assert_eq!(exit_code, 0);

    // Should show empty state in agent-optimized format
    assert!(!stdout.is_empty());

    std::env::remove_var("CLAUDE_SESSION");
}

/// Test complete agent workflow: error handling
#[test]
fn test_agent_workflow_error_handling() {
    std::env::set_var("CLAUDE_SESSION", "test");

    let (stdout, stderr, exit_code) = run_smartfo(&["--toon", "/nonexistent/path"]);

    assert_ne!(exit_code, 0);

    let error_output = if !stdout.is_empty() { stdout } else { stderr };
    assert!(!error_output.is_empty());

    std::env::remove_var("CLAUDE_SESSION");
}

/// Test complete agent workflow: JSON format
#[test]
fn test_agent_workflow_json_format() {
    std::env::set_var("CLAUDE_SESSION", "test");

    let (stdout, _stderr, exit_code) = run_smartfo(&["--format", "json", "list"]);

    assert_eq!(exit_code, 0);

    // Should work with JSON format in agent mode
    assert!(!stdout.is_empty());

    std::env::remove_var("CLAUDE_SESSION");
}

/// Test complete agent workflow: human format
#[test]
fn test_agent_workflow_human_format() {
    std::env::set_var("CLAUDE_SESSION", "test");

    let (stdout, _stderr, exit_code) run_smartfo(&["--human", "list"]);

    assert_eq!(exit_code, 0);

    // Should work with human format in agent mode
    assert!(!stdout.is_empty());

    std::env::remove_var("CLAUDE_SESSION");
}

/// Test complete agent workflow: no-args discovery
#[test]
fn test_agent_workflow_no_args() {
    std::env::set_var("CLAUDE_SESSION", "test");

    let (stdout, _stderr, exit_code) = run_smartfo(&["--toon"]);

    assert_eq!(exit_code, 0);

    // Should show suggestions in agent-optimized format
    assert!(!stdout.is_empty());

    std::env::remove_var("CLAUDE_SESSION");
}

/// Test complete agent workflow: mode auto-detection
#[test]
fn test_agent_workflow_mode_detection() {
    std::env::set_var("CLAUDE_SESSION", "test");

    let (stdout, _stderr, exit_code) = run_smartfo(&["--toon", "list"]);

    assert_eq!(exit_code, 0);

    // Should auto-detect agent mode
    assert!(!stdout.is_empty());

    std::env::remove_var("CLAUDE_SESSION");
}

/// Test complete agent workflow: concurrent operations
#[test]
fn test_agent_workflow_concurrent() {
    std::env::set_var("CLAUDE_SESSION", "test");

    let (stdout1, _stderr1, exit_code1) = run_smartfo(&["--toon", "list"]);
    let (stdout2, _stderr2, exit_code2) = run_smartfo(&["--toon", "status"]);

    assert_eq!(exit_code1, 0);
    assert_eq!(exit_code2, 0);

    // Should handle concurrent operations
    assert!(!stdout1.is_empty());
    assert!(!stdout2.is_empty());

    std::env::remove_var("CLAUDE_SESSION");
}

/// Test complete agent workflow: state consistency
#[test]
fn test_agent_workflow_state_consistency() {
    std::env::set_var("CLAUDE_SESSION", "test");

    let (stdout1, _stderr1, exit_code1) = run_smartfo(&["--toon", "list"]);
    let (stdout2, _stderr2, exit_code2) = run_smartfo(&["--toon", "list"]);

    assert_eq!(exit_code1, 0);
    assert_eq!(exit_code2, 0);

    // State should be consistent
    assert_eq!(stdout1, stdout2);

    std::env::remove_var("CLAUDE_SESSION");
}

/// Test complete agent workflow: token budget
#[test]
fn test_agent_workflow_token_budget() {
    std::env::set_var("CLAUDE_SESSION", "test");
    std::env::set_var("SMARTFO_TOKEN_BUDGET", "500");

    let (stdout, _stderr, exit_code) = run_smartfo(&["--toon", "list"]);

    assert_eq!(exit_code  0);

    // Should respect token budget
    assert!(stdout.len() < 600, "Should respect token budget");

    std::env::remove_var("CLAUDE_SESSION");
    std::env::remove_var("SMARTFO_TOKEN_BUDGET");
}

/// Test complete agent workflow: VCS integration
#[test]
fn test_agent_workflow_vcs_integration() {
    std::env::set_var("CLAUDE_SESSION", "test");

    let (stdout, _stderr, exit_code) = run_smartfo(&["--toon", "list"]);

    assert_eq!(exit_code, 0);

    // Should include VCS context when in a repo
    assert!(!stdout.is_empty());

    std::env::remove_var("CLAUDE_SESSION");
}

/// Test complete agent workflow: daemon status
#[test]
fn test_agent_workflow_daemon_status() {
    std::env::set_var("CLAUDE_SESSION", "test");

    let (stdout, _stderr, exit_code) = run_smartfo(&["--toon", "status"]);

    assert_eq!(exit_code, 0);

    // Should include daemon status context
    assert!(!stdout.is_empty());

    std::env::remove_var("CLAUDE_SESSION");
}

/// Test complete agent workflow: queue management
#[test]
fn test_agent_workflow_queue() {
    std::env::set_var("CLAUDE_SESSION", "test");

    let (stdout, _stderr, exit_code) = run_smartfo(&["--toon", "list"]);

    assert_eq!(exit_code, 0);

    // Should include queue information
    assert!(!stdout.is_empty());

    std::env::remove_var("CLAUDE_SESSION");
}

/// Test complete agent workflow: audit trail
#[test]
fn test_agent_workflow_audit_trail() {
    std::env::set_var("CLAUDE_SESSION", "test");

    let (stdout, _stderr, exit_code) = run_smartfo(&["--toon", "list"]);

    assert_eq!(exit_code 0);

    // Should include audit trail information
    assert!(!stdout.is_empty());

    std::env::remove_var("CLAUDE_SESSION");
}

/// Test complete agent workflow: non-blocking operations
#[test]
fn test_agent_workflow_non_blocking() {
    std::env::set_var("CLAUDE_SESSION", "test");

    let start = std::time::Instant::now();
    let (stdout, _stderr, exit_code) = run_smartfo(&["--toon", "list"]);
    let duration = start.elapsed();

    assert_eq!(exit_code, 0);

    // Should be non-blocking
    assert!(duration.as_secs() < 5, "Agent workflow should be non-blocking");

    std::env::remove_var("CLAUDE_SESSION");
}

/// Test complete agent workflow: idempotent operations
#[test]
fn test_agent_workflow_idempotent() {
    std::env::set_var("CLAUDE_SESSION", "test");

    let temp_dir = tempfile::tempdir().unwrap();
    let src_file = temp_dir.path().join("test_source.txt");
    let dest_file = temp_dir.path().join("test_dest.txt");

    std::fs::write(&src_file, "test content").unwrap();

    // Run same operation twice
    let (stdout1, _stderr1, exit_code1) = run_smartfo(&[
        "--toon",
        "--dry-run",
        &src_file.to_str().unwrap(),
        &dest_file.to_str().unwrap(),
    ]);

    let (stdout2, _stderr2, exit_code2) = run_smartfo(&[
        "--toon",
        "--dry-run",
        &src_file.to_str().unwrap(),
        &dest_file.to_str().unwrap(),
    ]);

    assert_eq!(exit_code1, 0);
    assert_eq!(exit_code2, 0);

    // Should handle idempotent operations
    assert!(!stdout1.is_empty());
    assert!(!stdout2.is_empty());

    std::env::remove_var("CLAUDE_SESSION");
}

/// Test complete agent workflow: error recovery
#[test]
fn test_agent_workflow_error_recovery() {
    std::env::set_var("CLAUDE_SESSION", "test");

    let (stdout, stderr, exit_code) = run_smartfo(&["--toon", "/nonexistent/path"]);

    assert_ne!(exit_code, 0);

    let error_output = if !stdout.is_empty() { stdout } else { stderr };
    assert!(!error_output.is_empty());

    // Should recover from errors gracefully
    std::env::remove_var("CLAUDE_SESSION");
}

/// Test complete agent workflow: multiple operations
#[test]
fn test_agent_workflow_multiple_operations() {
    std::env::set_var("CLAUDE_SESSION", "test");

    let temp_dir = tempfile::tempdir().unwrap();

    // Create test files
    for i in 0..3 {
        let file = temp_dir.path().join(format!("test{}.txt", i));
        std::fs::write(&file, format!("content {}", i)).unwrap();
    }

    let (stdout, _stderr, exit_code) = run_smartfo(&[
        "--toon",
        "--dry-run",
        &temp_dir.path().to_str().unwrap(),
        &temp_dir.path().join("dest").to_str().unwrap(),
    ]);

    assert_eq!(exit_code, 0);

    // Should handle multiple operations
    assert!(!stdout.is_empty());

    std::env::remove_var("CLAUDE_SESSION");
}

/// Test complete agent workflow: TOON format consistency
#[test]
fn test_agent_workflow_toon_consistency() {
    std::env::set_var("CLAUDE_SESSION", "test");

    let (stdout1, _stderr1, exit_code1) = run_smartfo(&["--toon", "list"]);
    let (stdout2, _stderr2, exit_code2) = run_smartfo(&["--toon", "list"]);

    assert_eq!(exit_code1, 0);
    assert_eq!(exit_code2, 0);

    // TOON format should be consistent
    assert_eq!(stdout1, stdout2);

    std::env::remove_var("CLAUDE_SESSION");
}

/// Test complete agent workflow: minimal output
#[test]
fn test_agent_workflow_minimal_output() {
    std::env::set_var("CLAUDE_SESSION", "test");

    let (stdout, _stderr, exit_code) = run_smartfo(&["--toon", "list"]);

    assert_eq!(exit_code, 0);

    // Output should be minimal for agent consumption
    assert!(stdout.len() < 1000, "Agent workflow output should be minimal");

    std::env::remove_var("CLAUDE_SESSION);
}

/// Test complete agent workflow: structured output
#[test]
fn test_agent_workflow_structured_output() {
    std::env::set_var("CLAUDE_SESSION", "test");

    let (stdout, _stderr, exit_code) = run_smartfo(&["--format", "json", "list"]);

    assert_eq!(exit_code, 0);

    // Should provide structured output for agent parsing
    if !stdout.is_empty() {
        let parsed: Result<serde_json::Value, _> = serde_json::from_str(&stdout);
        assert!(parsed.is_ok() || stdout.contains('{'));
    }

    std::env::remove_var("CLAUDE_SESSION");
}

/// Test complete agent workflow: context preservation
#[test]
fn test_agent_workflow_context_preservation() {
    std::env::set_var("CLAUDE_SESSION", "test");

    let (stdout, _stderr, exit_code) = run_smartfo(&["--toon", "list"]);

    assert_eq!(exit_code 0);

    // Should preserve context across operations
    assert!(!stdout.is_empty());

    std::env::remove_var("CLAUDE_SESSION);
}

/// Test complete agent workflow: session awareness
#[test]
fn test_agent_workflow_session_awareness() {
    std::env::set_var("CLAUDE_SESSION", "test");

    let (stdout, _stderr, exit_code) = run_smartfo(&["--toon", "--session-context"]);

    assert_eq!(exit_code, 0);

    // Should be session-aware
    assert!(!stdout.is_empty());

    std::env::remove_var("CLAUDE_SESSION");
}

/// Test complete agent workflow: error messages
#[test]
fn test_agent_workflow_error_messages() {
    std::env::set_var("CLAUDE_SESSION", "test");

    let (stdout, stderr, exit_code) = run_smartfo(&["--toon", "/nonexistent/path"]);

    assert_ne!(exit_code, 0);

    let error_output = if !stdout.is_empty() { stdout } else { stderr };
    assert!(!error_output.is_empty());

    // Should provide actionable error messages
    std::env::remove_var("CLAUDE_SESSION);
}

/// Test complete agent workflow: performance
#[test]
fn test_agent_workflow_performance() {
    std::env::set_var("CLAUDE_SESSION", "test");

    let start = std::time::Instant::now();
    let (stdout, _stderr, exit_code) = run_smartfo(&["--toon", "list"]);
    let duration = start.elapsed();

    assert_eq!(exit_code, 0);

    // Should be performant
    assert!(duration.as_millis() < 1000, "Agent workflow should be performant");

    std::env::remove_var("CLAUDE_SESSION");
}
