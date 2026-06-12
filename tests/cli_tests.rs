use assert_cmd::Command;
use assert_cmd::assert::OutputAssertExt;
use predicates::prelude::*;

/// Create a symlink to the smartfo binary with the given name.
fn symlink_binary(name: &str) -> tempfile::TempDir {
    let tmp = tempfile::tempdir().unwrap();
    let bin = Command::cargo_bin("smartfo").unwrap();
    let dest = tmp.path().join(name);
    #[cfg(unix)]
    std::os::unix::fs::symlink(bin.get_program(), &dest).unwrap();
    #[cfg(windows)]
    std::os::windows::fs::symlink_file(bin.get_program(), &dest).unwrap();
    tmp
}

#[test]
fn test_smartfo_help() {
    let mut cmd = Command::cargo_bin("smartfo").unwrap();
    cmd.arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Usage:"));
}

#[test]
fn test_smartfo_version() {
    let mut cmd = Command::cargo_bin("smartfo").unwrap();
    cmd.arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains("smartfo"));
}

#[test]
fn test_smartfo_usage() {
    let mut cmd = Command::cargo_bin("smartfo").unwrap();
    cmd.arg("--usage")
        .assert()
        .success()
        .stdout(predicate::str::contains("Usage:"));
}

#[test]
fn test_smartfo_no_args_shows_content_first_summary() {
    let mut cmd = Command::cargo_bin("smartfo").unwrap();
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("current_directory"));
}

#[test]
fn test_daemon_flag_accepted() {
    let mut cmd = Command::cargo_bin("smartfo").unwrap();
    cmd.arg("--daemon")
        .arg("--help")
        .assert()
        .success();
}

#[test]
fn test_no_daemon_flag_accepted() {
    let mut cmd = Command::cargo_bin("smartfo").unwrap();
    cmd.arg("--no-daemon")
        .arg("--help")
        .assert()
        .success();
}

#[test]
fn test_job_list_command_accepted() {
    let mut cmd = Command::cargo_bin("smartfo").unwrap();
    cmd.arg("job")
        .arg("list")
        .assert()
        .success();
}

#[test]
fn test_job_list_with_ids_accepted() {
    let mut cmd = Command::cargo_bin("smartfo").unwrap();
    cmd.arg("job")
        .arg("list")
        .arg("--ids")
        .arg("123e4567-e89b-12d3-a456-426614174000")
        .assert()
        .success();
}

#[test]
fn test_job_cancel_command_accepted() {
    let mut cmd = Command::cargo_bin("smartfo").unwrap();
    cmd.arg("job")
        .arg("cancel")
        .arg("123e4567-e89b-12d3-a456-426614174000")
        .assert()
        .success();
}

#[test]
fn test_smartfo_install_flag() {
    let mut cmd = Command::cargo_bin("smartfo").unwrap();
    cmd.arg("--install")
        .assert()
        .success()
        .stderr(predicate::str::contains("install mode"));
}

#[test]
fn test_smartfo_git_hook_client() {
    let mut cmd = Command::cargo_bin("smartfo").unwrap();
    cmd.arg("git")
        .arg("hook-client")
        .assert()
        .success()
        .stderr(predicate::str::contains("pre-commit"));
}

#[test]
fn test_smartfo_git_hook_server() {
    let mut cmd = Command::cargo_bin("smartfo").unwrap();
    cmd.arg("git")
        .arg("hook-server")
        .assert()
        .failure()
        .stderr(predicate::str::contains("pre-receive"));
}

#[test]
fn test_mv_symlink_help() {
    let tmp = symlink_binary("mv");
    let mut cmd = std::process::Command::new(tmp.path().join("mv"));
    cmd.arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Usage:"));
}

#[test]
fn test_mv_symlink_version() {
    let tmp = symlink_binary("mv");
    let mut cmd = std::process::Command::new(tmp.path().join("mv"));
    cmd.arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains("mv"));
}

#[test]
fn test_mv_symlink_usage() {
    let tmp = symlink_binary("mv");
    let mut cmd = std::process::Command::new(tmp.path().join("mv"));
    cmd.arg("--usage")
        .assert()
        .success()
        .stdout(predicate::str::contains("Usage:"));
}

#[test]
fn test_mv_dry_run() {
    let tmp = symlink_binary("mv");
    let mut cmd = std::process::Command::new(tmp.path().join("mv"));
    cmd.arg("--dry-run")
        .arg("/tmp/fake-src")
        .arg("/tmp/fake-dest")
        .assert()
        .success()
        .stderr(predicate::str::contains("dry-run: mv"));
}

#[test]
fn test_mv_missing_operand() {
    let tmp = symlink_binary("mv");
    let mut cmd = std::process::Command::new(tmp.path().join("mv"));
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("missing file operand"));
}

#[test]
fn test_rm_symlink_help() {
    let tmp = symlink_binary("rm");
    let mut cmd = std::process::Command::new(tmp.path().join("rm"));
    cmd.arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Usage:"));
}

#[test]
fn test_rm_symlink_version() {
    let tmp = symlink_binary("rm");
    let mut cmd = std::process::Command::new(tmp.path().join("rm"));
    cmd.arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains("rm"));
}

#[test]
fn test_rm_symlink_usage() {
    let tmp = symlink_binary("rm");
    let mut cmd = std::process::Command::new(tmp.path().join("rm"));
    cmd.arg("--usage")
        .assert()
        .success()
        .stdout(predicate::str::contains("Usage:"));
}

#[test]
fn test_rm_dry_run() {
    let tmp = symlink_binary("rm");
    let mut cmd = std::process::Command::new(tmp.path().join("rm"));
    cmd.arg("--dry-run")
        .arg("/tmp/fake-file")
        .assert()
        .success()
        .stderr(predicate::str::contains("dry-run: rm"));
}

#[test]
fn test_rm_missing_operand() {
    let tmp = symlink_binary("rm");
    let mut cmd = std::process::Command::new(tmp.path().join("rm"));
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("missing operand"));
}

#[test]
fn test_smv_dispatch() {
    let tmp = symlink_binary("smv");
    let mut cmd = std::process::Command::new(tmp.path().join("smv"));
    cmd.arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Move"));
}

#[test]
fn test_srm_dispatch() {
    let tmp = symlink_binary("srm");
    let mut cmd = std::process::Command::new(tmp.path().join("srm"));
    cmd.arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Remove"));
}

#[test]
fn test_mv_flags_parsed() {
    let tmp = symlink_binary("mv");
    let mut cmd = std::process::Command::new(tmp.path().join("mv"));
    // These flags should be accepted without error (avoiding conflicting flags)
    cmd.arg("-f")
        .arg("-v")
        .arg("--plain")
        .arg("--async")
        .arg("--sync")
        .arg("--dry-run")
        .arg("--force-outside-vcs")
        .arg("/tmp/fake-src")
        .arg("/tmp/fake-dest")
        .assert()
        .success();
}

#[test]
fn test_rm_flags_parsed() {
    let tmp = symlink_binary("rm");
    let mut cmd = std::process::Command::new(tmp.path().join("rm"));
    // These flags should be accepted without error (avoiding conflicting flags)
    cmd.arg("-f")
        .arg("-r")
        .arg("--plain")
        .arg("--force-delete")
        .arg("--sync")
        .arg("--dry-run")
        .arg("/tmp/fake-file")
        .assert()
        .success();
}

// Tests for hierarchical subcommand structure
#[test]
fn test_smartfo_git_subcommand_help() {
    let mut cmd = Command::cargo_bin("smartfo").unwrap();
    cmd.arg("git")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Git hook commands"));
}

#[test]
fn test_smartfo_job_subcommand_help() {
    let mut cmd = Command::cargo_bin("smartfo").unwrap();
    cmd.arg("job")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Job management commands"));
}

#[test]
fn test_smartfo_agent_subcommand_help() {
    let mut cmd = Command::cargo_bin("smartfo").unwrap();
    cmd.arg("agent")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Agent integration commands"));
}

#[test]
fn test_smartfo_info_subcommand_help() {
    let mut cmd = Command::cargo_bin("smartfo").unwrap();
    cmd.arg("info")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Information and query commands"));
}

#[test]
fn test_smartfo_job_list() {
    let mut cmd = Command::cargo_bin("smartfo").unwrap();
    cmd.arg("job")
        .arg("list")
        .assert()
        .success()
        .stdout(predicate::str::contains("Background job listing"));
}

#[test]
fn test_smartfo_job_cancel() {
    let mut cmd = Command::cargo_bin("smartfo").unwrap();
    cmd.arg("job")
        .arg("cancel")
        .arg("test-job-id")
        .assert()
        .success()
        .stdout(predicate::str::contains("Cancelling job"));
}

#[test]
fn test_smartfo_agent_session_context() {
    let mut cmd = Command::cargo_bin("smartfo").unwrap();
    cmd.arg("agent")
        .arg("session-context")
        .assert()
        .success()
        .stdout(predicate::str::contains("(cwd"));
}

#[test]
fn test_smartfo_agent_install_hooks() {
    let mut cmd = Command::cargo_bin("smartfo").unwrap();
    cmd.arg("agent")
        .arg("install-hooks")
        .assert()
        .success()
        .stdout(predicate::str::contains("Agent hooks installed"));
}

#[test]
fn test_smartfo_agent_generate_skill() {
    let mut cmd = Command::cargo_bin("smartfo").unwrap();
    cmd.arg("agent")
        .arg("generate-skill")
        .assert()
        .success()
        .stdout(predicate::str::contains("name: smartfo"));
}

#[test]
fn test_smartfo_info_list() {
    let mut cmd = Command::cargo_bin("smartfo").unwrap();
    cmd.arg("info")
        .arg("list")
        .assert()
        .success()
        .stdout(predicate::str::contains("items"));
}

#[test]
fn test_smartfo_info_status() {
    let mut cmd = Command::cargo_bin("smartfo").unwrap();
    cmd.arg("info")
        .arg("status")
        .assert()
        .success()
        .stdout(predicate::str::contains("status"));
}

#[test]
fn test_smartfo_help_shows_hierarchy() {
    let mut cmd = Command::cargo_bin("smartfo").unwrap();
    cmd.arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("git"))
        .stdout(predicate::str::contains("job"))
        .stdout(predicate::str::contains("agent"))
        .stdout(predicate::str::contains("info"));
}

#[test]
fn test_smartfo_usage_shows_hierarchy() {
    let mut cmd = Command::cargo_bin("smartfo").unwrap();
    cmd.arg("--usage")
        .assert()
        .success()
        .stdout(predicate::str::contains("git"))
        .stdout(predicate::str::contains("job"))
        .stdout(predicate::str::contains("agent"))
        .stdout(predicate::str::contains("info"));
}
