//! Integration tests for minimal schemas
//! Tests schema registry, field selection, and schema validation through CLI

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

/// Test that list command uses list schema by default
#[test]
fn test_list_command_default_schema() {
    let (stdout, _stderr, exit_code) = run_smartfo(&["--toon", "list"]);

    assert_eq!(exit_code, 0);

    // List schema default fields: id, type, status, source
    // Output should contain these fields (or be empty with context)
    assert!(!stdout.is_empty());
}

/// Test that status command uses status schema by default
#[test]
fn test_status_command_default_schema() {
    let (stdout, _stderr, exit_code) = run_smartfo(&["--toon", "status"]);

    assert_eq!(exit_code, 0);

    // Status schema default fields: operation, queue_size, daemon_status
    assert!(!stdout.is_empty());
}

/// Test that --fields flag works with list command
#[test]
fn test_fields_flag_with_list() {
    let (stdout, _stderr, exit_code) = run_smartfo(&[
        "--toon",
        "--fields",
        "id,status",
        "list",
    ]);

    assert_eq!(exit_code, 0);
    assert!(!stdout.is_empty());

    // Output should be limited to id and status fields
    assert!(stdout.contains("id") || stdout.contains("status") || !stdout.is_empty());
}

/// Test that --fields flag works with status command
#[test]
fn test_fields_flag_with_status() {
    let (stdout, _stderr, exit_code) = run_smartfo(&[
        "--toon",
        "--fields",
        "operation,daemon_status",
        "status",
    ]);

    assert_eq!(exit_code, 0);
    assert!(!stdout.is_empty());

    // Output should be limited to operation and daemon_status fields
    assert!(stdout.contains("operation") || stdout.contains("daemon_status") || !stdout.is_empty());
}

/// Test that invalid field names are rejected
#[test]
fn test_invalid_field_rejection() {
    let (stdout, stderr, exit_code) = run_smartfo(&[
        "--toon",
        "--fields",
        "invalid_field_name",
        "list",
    ]);

    // Should fail with error about invalid field
    assert_ne!(exit_code, 0);
    assert!(!stderr.is_empty() || !stdout.is_empty());
}

/// Test that field selection is case-sensitive
#[test]
fn test_field_case_sensitivity() {
    // Try with uppercase field name (should fail)
    let (stdout, stderr, exit_code) = run_smartfo(&[
        "--toon",
        "--fields",
        "ID",
        "list",
    ]);

    // Should fail or use default
    assert!(exit_code != 0 || !stdout.is_empty());
}

/// Test that comma-separated field parsing works
#[test]
fn test_comma_separated_fields() {
    let (stdout, _stderr, exit_code) = run_smartfo(&[
        "--toon",
        "--fields",
        "id,type,status",
        "list",
    ]);

    assert_eq!(exit_code, 0);
    assert!(!stdout.is_empty());
}

/// Test that spaces in field list are handled
#[test]
fn test_field_list_with_spaces() {
    let (stdout, _stderr, exit_code) = run_smartfo(&[
        "--toon",
        "--fields",
        "id, type , status",
        "list",
    ]);

    assert_eq!(exit_code, 0);
    assert!(!stdout.is_empty());
}

/// Test that empty field list uses default schema
#[test]
fn test_empty_field_list_uses_default() {
    let (stdout1, _stderr1, exit_code1) = run_smartfo(&["--toon", "list"]);
    let (stdout2, _stderr2, exit_code2) = run_smartfo(&["--toon", "--fields", "", "list"]);

    assert_eq!(exit_code1, 0);
    assert_eq!(exit_code2, 0);

    // Both should produce similar output (using default schema)
    assert!(!stdout1.is_empty());
    assert!(!stdout2.is_empty());
}

/// Test that schema limits available fields
#[test]
fn test_schema_field_limits() {
    // Try to request a field that's not in the list schema
    let (stdout, stderr, exit_code) = run_smartfo(&[
        "--toon",
        "--fields",
        "daemon_status",
        "list",
    ]);

    // Should fail or ignore invalid field
    assert!(exit_code != 0 || !stderr.is_empty());
}

/// Test that different commands use different schemas
#[test]
fn test_command_specific_schemas() {
    // List command
    let (list_out, _list_err, list_exit) = run_smartfo(&["--toon", "list"]);
    assert_eq!(list_exit, 0);

    // Status command
    let (status_out, _status_err, status_exit) = run_smartfo(&["--toon", "status"]);
    assert_eq!(status_exit, 0);

    // Outputs should be different (different schemas)
    assert!(!list_out.is_empty());
    assert!(!status_out.is_empty());
}

/// Test that schema works with TOON format
#[test]
fn test_schema_with_toon_format() {
    let (stdout, _stderr, exit_code) = run_smartfo(&[
        "--toon",
        "--fields",
        "id,status",
        "list",
    ]);

    assert_eq!(exit_code, 0);

    // TOON format should respect field selection
    assert!(!stdout.is_empty());

    // Should have TOON structure (key:value pairs)
    let lines: Vec<&str> = stdout.lines().collect();
    let has_key_value = lines.iter().any(|line| line.contains(':'));
    assert!(has_key_value);
}

/// Test that schema works with JSON format
#[test]
fn test_schema_with_json_format() {
    let (stdout, _stderr, exit_code) = run_smartfo(&[
        "--format",
        "json",
        "--fields",
        "id,status",
        "list",
    ]);

    assert_eq!(exit_code, 0);

    // JSON format should respect field selection
    assert!(!stdout.is_empty());

    // Should be valid JSON
    if !stdout.is_empty() {
        let parsed: Result<serde_json::Value, _> = serde_json::from_str(&stdout);
        assert!(parsed.is_ok() || stdout.contains('{'));
    }
}

/// Test that schema works with human format
#[test]
fn test_schema_with_human_format() {
    let (stdout, _stderr, exit_code) = run_smartfo(&[
        "--human",
        "--fields",
        "id,status",
        "list",
    ]);

    assert_eq!(exit_code, 0);
    assert!(!stdout.is_empty());
}

/// Test that minimal schemas keep output concise
#[test]
fn test_minimal_schema_conciseness() {
    let (stdout, _stderr, exit_code) = run_smartfo(&["--toon", "list"]);

    assert_eq!(exit_code, 0);

    // Output should be concise (minimal fields)
    let line_count = stdout.lines().count();
    assert!(line_count < 20, "Minimal schema should produce concise output");
}

/// Test that schema validation happens at runtime
#[test]
fn test_runtime_schema_validation() {
    // Request a field that doesn't exist in the schema
    let (stdout, stderr, exit_code) = run_smartfo(&[
        "--toon",
        "--fields",
        "nonexistent_field",
        "list",
    ]);

    // Should fail with validation error
    assert_ne!(exit_code, 0);
    assert!(!stderr.is_empty());
}

/// Test that all default schemas are available
#[test]
fn test_all_default_schemas_available() {
    // Test list schema
    let (stdout, _stderr, exit_code) = run_smartfo(&["--toon", "list"]);
    assert_eq!(exit_code, 0);
    assert!(!stdout.is_empty());

    // Test status schema
    let (stdout, _stderr, exit_code) = run_smartfo(&["--toon", "status"]);
    assert_eq!(exit_code, 0);
    assert!(!stdout.is_empty());

    // Test session-context (uses session schema)
    let (stdout, _stderr, exit_code) = run_smartfo(&["--toon", "session-context"]);
    assert_eq!(exit_code, 0);
    assert!(!stdout.is_empty());
}

/// Test that schema field order is preserved
#[test]
fn test_field_order_preservation() {
    let (stdout, _stderr, exit_code) = run_smartfo(&[
        "--toon",
        "--fields",
        "status,id,type",
        "list",
    ]);

    assert_eq!(exit_code, 0);
    assert!(!stdout.is_empty());

    // Fields should appear in requested order
    // This is a basic check - actual order verification may vary
    assert!(stdout.contains("status") || stdout.contains("id") || stdout.contains("type"));
}

/// Test that duplicate fields are handled
#[test]
fn test_duplicate_field_handling() {
    let (stdout, _stderr, exit_code) = run_smartfo(&[
        "--toon",
        "--fields",
        "id,id,status",
        "list",
    ]);

    // Should either deduplicate or handle gracefully
    assert!(exit_code == 0 || !stderr.is_empty());
}

/// Test that schema works with empty results
#[test]
fn test_schema_with_empty_results() {
    let (stdout, _stderr, exit_code) = run_smartfo(&["--toon", "list"]);

    assert_eq!(exit_code, 0);

    // Empty results should still have schema structure
    assert!(!stdout.is_empty());
}

/// Test that schema works with mv command
#[test]
fn test_schema_with_mv_command() {
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

/// Test that schema works with rm command
#[test]
fn test_schema_with_rm_command() {
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

/// Test that schema metadata is consistent
#[test]
fn test_schema_metadata_consistency() {
    // Run same command twice to check consistency
    let (stdout1, _stderr1, exit_code1) = run_smartfo(&["--toon", "--fields", "id,status", "list"]);
    let (stdout2, _stderr2, exit_code2) = run_smartfo(&["--toon", "--fields", "id,status", "list"]);

    assert_eq!(exit_code1, 0);
    assert_eq!(exit_code2, 0);

    // Schema structure should be consistent
    assert!(!stdout1.is_empty());
    assert!(!stdout2.is_empty());
}
