use assert_cmd::Command;
use assert_cmd::assert::OutputAssertExt;
use predicates::prelude::*;
use std::path::Path;

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
fn test_smartfo_no_args_shows_help() {
    let mut cmd = Command::cargo_bin("smartfo").unwrap();
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Usage:"));
}

#[test]
fn test_smartfo_install_flag() {
    let mut cmd = Command::cargo_bin("smartfo").unwrap();
    cmd.arg("--install")
        .assert()
        .success()
        .stdout(predicate::str::contains("install mode"));
}

#[test]
fn test_smartfo_git_hook_client() {
    let mut cmd = Command::cargo_bin("smartfo").unwrap();
    cmd.arg("git-hook-client")
        .assert()
        .success()
        .stderr(predicate::str::contains("git-hook-client"));
}

#[test]
fn test_smartfo_git_hook_server() {
    let mut cmd = Command::cargo_bin("smartfo").unwrap();
    cmd.arg("git-hook-server")
        .assert()
        .success()
        .stderr(predicate::str::contains("git-hook-server"));
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
    // All these flags should be accepted without error
    cmd.arg("-f")
        .arg("-i")
        .arg("-n")
        .arg("-v")
        .arg("--plain")
        .arg("--async")
        .arg("--blocking")
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
    // All these flags should be accepted without error
    cmd.arg("-f")
        .arg("-i")
        .arg("-I")
        .arg("-r")
        .arg("-d")
        .arg("--plain")
        .arg("--force-delete")
        .arg("--blocking")
        .arg("--sync")
        .arg("--dry-run")
        .arg("/tmp/fake-file")
        .assert()
        .success();
}
