use sls::*;
use assert_cmd::Command;
use predicates::str::contains;
use predicates::prelude::PredicateBooleanExt;

#[test]
fn test_parse_size() {
    assert_eq!(parse_size("10KB"), Some(10 * 1024));
}

#[test]
fn test_basic_list() {
    let mut cmd = Command::cargo_bin("sls").unwrap();
    cmd.assert().success().stdout(predicates::str::contains("src"));
}

#[test]
fn test_extension_filter() {
    let mut cmd = Command::cargo_bin("sls").unwrap();
    cmd.arg("--ext").arg("rs");
    cmd.assert().success().stdout(predicates::str::contains(".rs"));
}

#[test]
fn test_invalid_date_range() {
    let mut cmd = Command::cargo_bin("sls").unwrap();
    cmd.arg("--modified").arg("badrange");
    cmd.assert()
        .failure()
        .code(1)
        .stderr(contains("Invalid date range format for --modified").or(contains("Expected YYYY-MM-DD..YYYY-MM-DD")));
}