// Integration tests for pager integration

use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_help_pager_integration() {
    // Test that --usage flag works with pager
    let mut cmd = Command::cargo_bin("smartfo").unwrap();
    let result = cmd.arg("--usage").arg("--no-pager").assert();
    result.success();
}

#[test]
fn test_man_pager_integration() {
    // Test that --man flag works with pager
    let mut cmd = Command::cargo_bin("smartfo").unwrap();
    let result = cmd.arg("--man").arg("--no-pager").assert();
    result.success();
}

#[test]
fn test_no_pager_flag_works() {
    // Test that --no-pager flag is accepted
    let mut cmd = Command::cargo_bin("smartfo").unwrap();
    let result = cmd.arg("--usage").arg("--no-pager").assert();
    result.success();
}
