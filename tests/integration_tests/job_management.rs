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
