//! Integration tests for CLI commands.

use assert_cmd::Command;
use predicates::prelude::*;

/// Test that --help flag displays usage information.
#[test]
fn test_help_flag() {
    let mut cmd = Command::cargo_bin("ports").unwrap();
    cmd.arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("リッスン中のTCPポートとアプリケーション情報を表示"));
}

/// Test that --version flag displays version.
#[test]
fn test_version_flag() {
    let mut cmd = Command::cargo_bin("ports").unwrap();
    cmd.arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains("ports"));
}

/// Test that invalid flag returns error.
#[test]
fn test_invalid_flag() {
    let mut cmd = Command::cargo_bin("ports").unwrap();
    cmd.arg("--invalid-flag")
        .assert()
        .failure()
        .stderr(predicate::str::contains("error"));
}

/// Test basic execution (may show ports or empty message).
#[test]
fn test_basic_execution() {
    let mut cmd = Command::cargo_bin("ports").unwrap();
    cmd.assert().success();
}

/// Test JSON output format.
#[test]
fn test_json_output() {
    let mut cmd = Command::cargo_bin("ports").unwrap();
    cmd.arg("--json")
        .assert()
        .success()
        .stdout(predicate::str::contains("\"ports\""))
        .stdout(predicate::str::contains("\"total_count\""))
        .stdout(predicate::str::contains("\"timestamp\""));
}

/// Test short JSON flag.
#[test]
fn test_short_json_flag() {
    let mut cmd = Command::cargo_bin("ports").unwrap();
    cmd.arg("-j")
        .assert()
        .success()
        .stdout(predicate::str::contains("\"ports\""));
}
