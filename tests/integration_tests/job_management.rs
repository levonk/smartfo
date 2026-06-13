use assert_cmd::Command;
use assert_cmd::assert::OutputAssertExt;
use predicates::prelude::*;
use tempfile::TempDir;
use std::path::PathBuf;
use uuid::Uuid;

#[test]
fn test_job_list_empty_queue() {
    let mut cmd = Command::cargo_bin("smartfo").unwrap();
    cmd.arg("job")
        .arg("list")
        .assert()
        .success()
        .stdout(predicate::str::contains("No jobs found"));
}

#[test]
fn test_job_list_with_invalid_uuid() {
    let mut cmd = Command::cargo_bin("smartfo").unwrap();
    cmd.arg("job")
        .arg("list")
        .arg("--ids")
        .arg("invalid-uuid")
        .assert()
        .success(); // Should succeed but show no jobs
}

#[test]
fn test_job_cancel_invalid_uuid() {
    let mut cmd = Command::cargo_bin("smartfo").unwrap();
    cmd.arg("job")
        .arg("cancel")
        .arg("invalid-uuid")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Invalid job ID format"));
}

#[test]
fn test_job_cancel_nonexistent_uuid() {
    let random_uuid = Uuid::new_v4().to_string();
    let mut cmd = Command::cargo_bin("smartfo").unwrap();
    cmd.arg("job")
        .arg("cancel")
        .arg(&random_uuid)
        .assert()
        .success()
        .stdout(predicate::str::contains("not found"));
}

#[test]
fn test_daemon_flag_with_help() {
    let mut cmd = Command::cargo_bin("smartfo").unwrap();
    cmd.arg("--daemon")
        .arg("--help")
        .assert()
        .success();
}

#[test]
fn test_no_daemon_flag_with_help() {
    let mut cmd = Command::cargo_bin("smartfo").unwrap();
    cmd.arg("--no-daemon")
        .arg("--help")
        .assert()
        .success();
}

#[test]
fn test_job_list_with_single_id_filter() {
    let random_uuid = Uuid::new_v4().to_string();
    let mut cmd = Command::cargo_bin("smartfo").unwrap();
    cmd.arg("job")
        .arg("list")
        .arg("--ids")
        .arg(&random_uuid)
        .assert()
        .success()
        .stdout(predicate::str::contains("No jobs found")); // UUID doesn't exist yet
}

#[test]
fn test_job_list_with_multiple_id_filter() {
    let uuid1 = Uuid::new_v4().to_string();
    let uuid2 = Uuid::new_v4().to_string();
    let ids = format!("{},{}", uuid1, uuid2);

    let mut cmd = Command::cargo_bin("smartfo").unwrap();
    cmd.arg("job")
        .arg("list")
        .arg("--ids")
        .arg(&ids)
        .assert()
        .success()
        .stdout(predicate::str::contains("No jobs found")); // UUIDs don't exist yet
}

#[test]
fn test_job_list_quiet_mode() {
    let mut cmd = Command::cargo_bin("smartfo").unwrap();
    cmd.arg("job")
        .arg("list")
        .arg("--quiet")
        .assert()
        .success();
}

#[test]
fn test_job_list_debug_mode() {
    let mut cmd = Command::cargo_bin("smartfo").unwrap();
    cmd.arg("job")
        .arg("list")
        .arg("--debug")
        .assert()
        .success();
}

#[test]
fn test_daemon_platform_fallback_unix() {
    #[cfg(unix)]
    {
        // On Unix, daemon should be supported
        let mut cmd = Command::cargo_bin("smartfo").unwrap();
        cmd.arg("--daemon")
            .arg("--help")
            .assert()
            .success();
    }

    #[cfg(not(unix))]
    {
        // On non-Unix, daemon should still accept the flag but may warn
        let mut cmd = Command::cargo_bin("smartfo").unwrap();
        cmd.arg("--daemon")
            .arg("--help")
            .assert()
            .success();
    }
}

#[test]
fn test_no_daemon_platform_fallback() {
    // --no-daemon should work on all platforms
    let mut cmd = Command::cargo_bin("smartfo").unwrap();
    cmd.arg("--no-daemon")
        .arg("--help")
        .assert()
        .success();
}

#[test]
fn test_secret_sanitization_in_audit() {
    use smartfo::audit::AuditEntry;

    // Create an audit entry with a secret in the reason field
    let entry = AuditEntry::new_move(
        "/tmp/source.txt".to_string(),
        "/tmp/dest.txt".to_string(),
        Some("API key: sk_live_1234567890abcdef for testing".to_string()),
        None,
        None,
        None,
    );

    // Serialize to JSONL
    let json = entry.to_jsonl().unwrap();

    // The secret should be sanitized
    assert!(json.contains("sk_********"));
    assert!(!json.contains("sk_live_1234567890abcdef"));
}

#[test]
fn test_secret_sanitization_in_paths() {
    use smartfo::audit::AuditEntry;

    // Create an audit entry with a password in the path
    let entry = AuditEntry::new_move(
        "https://user:password@example.com/file.txt".to_string(),
        "/tmp/dest.txt".to_string(),
        None,
        None,
        None,
        None,
    );

    // Serialize to JSONL
    let json = entry.to_jsonl().unwrap();

    // The password should be sanitized
    assert!(json.contains("********@"));
    assert!(!json.contains("password@"));
}
