use assert_cmd::Command;
use predicates::str::contains;
use predicates::prelude::PredicateBooleanExt;
use std::fs;
use tempfile;

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

#[test]
fn test_lists_current_dir() {
    let mut cmd = Command::cargo_bin("sls").unwrap();
    cmd.assert()
        .success()
        .stdout(contains("src").or(contains("Cargo.toml")));
}

#[test]
fn test_extension_filter_rs() {
    let mut cmd = Command::cargo_bin("sls").unwrap();
    cmd.arg("--ext").arg("rs");
    cmd.assert()
        .success()
        .stdout(contains(".rs"))
        .stdout(predicates::str::contains("main").or(contains("lib")));
}

#[test]
fn test_hidden_files_included() {
    let tmp_dir = tempfile::tempdir().unwrap();
    let hidden_path = tmp_dir.path().join(".hiddenfile");
    fs::write(&hidden_path, "test").unwrap();

    let mut cmd = Command::cargo_bin("sls").unwrap();
    cmd.current_dir(tmp_dir.path())
        .arg("--hidden");
    cmd.assert()
        .success()
        .stdout(contains(".hiddenfile"));
}

#[test]
fn test_min_size_filter_excludes_small_files() {
    let tmp_dir = tempfile::tempdir().unwrap();
    let small_file = tmp_dir.path().join("small.txt");
    fs::write(&small_file, "tiny").unwrap();

    let mut cmd = Command::cargo_bin("sls").unwrap();
    cmd.current_dir(tmp_dir.path())
        .arg("--min-size").arg("10");
    cmd.assert()
        .success()
        .stdout(predicates::str::is_empty());
}

#[test]
fn test_json_output_format() {
    let mut cmd = Command::cargo_bin("sls").unwrap();
    cmd.arg("--json");
    cmd.assert()
        .success()
        .stdout(contains("\"path\""))
        .stdout(contains("\"size\""));
}