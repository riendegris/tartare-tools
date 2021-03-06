use assert_cmd::prelude::*;
use std::process::Command;
use tempfile::TempDir;
use transit_model::test_utils::*;

#[test]
fn test_extract_network() {
    let output_dir = TempDir::new().expect("create temp dir failed");
    Command::cargo_bin("filter-ntfs")
        .expect("Failed to find binary 'filter-ntfs'")
        .arg("--input")
        .arg("tests/fixtures/input/")
        .arg("--output")
        .arg(output_dir.path().to_str().unwrap())
        .arg("extract")
        .arg("-n")
        .arg("network_id:network1")
        .arg("--current-datetime")
        .arg("2019-04-03T17:19:00+00:00")
        .assert()
        .success();
    compare_output_dir_with_expected(&output_dir, None, "tests/fixtures/output_extract");
}

#[test]
fn test_remove_network() {
    let output_dir = TempDir::new().expect("create temp dir failed");
    Command::cargo_bin("filter-ntfs")
        .expect("Failed to find binary 'filter-ntfs'")
        .arg("--input")
        .arg("tests/fixtures/input/")
        .arg("--output")
        .arg(output_dir.path().to_str().unwrap())
        .arg("remove")
        .arg("-n")
        .arg("network_id:network1")
        .arg("--current-datetime")
        .arg("2019-04-03T17:19:00+00:00")
        .assert()
        .success();
    compare_output_dir_with_expected(&output_dir, None, "tests/fixtures/output_remove");
}

#[test]
fn test_extract_with_unknown_network() {
    let output_dir = TempDir::new().expect("create temp dir failed");
    Command::cargo_bin("filter-ntfs")
        .expect("Failed to find binary 'filter-ntfs'")
        .arg("--input")
        .arg("tests/fixtures/input/")
        .arg("--output")
        .arg(output_dir.path().to_str().unwrap())
        .arg("remove")
        .arg("-n")
        .arg("network_id:unknown")
        .assert()
        .failure()
        .stderr(predicates::str::contains("Network \'unknown\' not found"));
}

#[test]
fn test_remove_all_networks() {
    let output_dir = TempDir::new().expect("create temp dir failed");
    Command::cargo_bin("filter-ntfs")
        .expect("Failed to find binary 'filter-ntfs'")
        .arg("--input")
        .arg("tests/fixtures/input/")
        .arg("--output")
        .arg(output_dir.path().to_str().unwrap())
        .arg("remove")
        .arg("-n")
        .arg("network_id:network1")
        .arg("network_id:network2")
        .arg("network_id:network3")
        .assert()
        .failure()
        .stderr(predicates::str::contains(
            "the data does not contain vehicle journeys anymore.",
        ));
}

#[test]
fn test_remove_line_by_line_code() {
    let output_dir = TempDir::new().expect("create temp dir failed");
    Command::cargo_bin("filter-ntfs")
        .expect("Failed to find binary 'filter-ntfs'")
        .arg("--input")
        .arg("tests/fixtures/input/")
        .arg("--output")
        .arg(output_dir.path().to_str().unwrap())
        .arg("remove")
        .arg("-l")
        .arg("line_code:route3")
        .arg("--current-datetime")
        .arg("2019-04-03T17:19:00+00:00")
        .assert()
        .success();
    compare_output_dir_with_expected(&output_dir, None, "tests/fixtures/output_remove_line");
}

#[test]
fn test_remove_line_by_line_id() {
    let output_dir = TempDir::new().expect("create temp dir failed");
    Command::cargo_bin("filter-ntfs")
        .expect("Failed to find binary 'filter-ntfs'")
        .arg("--input")
        .arg("tests/fixtures/input/")
        .arg("--output")
        .arg(output_dir.path().to_str().unwrap())
        .arg("remove")
        .arg("-l")
        .arg("line_id:line3")
        .arg("--current-datetime")
        .arg("2019-04-03T17:19:00+00:00")
        .assert()
        .success();
    compare_output_dir_with_expected(&output_dir, None, "tests/fixtures/output_remove_line");
}

#[test]
fn test_extract_multiple_line_by_line_code() {
    let output_dir = TempDir::new().expect("create temp dir failed");
    Command::cargo_bin("filter-ntfs")
        .expect("Failed to find binary 'filter-ntfs'")
        .arg("--input")
        .arg("tests/fixtures/input/")
        .arg("--output")
        .arg(output_dir.path().to_str().unwrap())
        .arg("extract")
        .arg("-l")
        .arg("line_code:route1")
        .arg("line_code:route3")
        .arg("--current-datetime")
        .arg("2019-04-03T17:19:00+00:00")
        .assert()
        .success();
    compare_output_dir_with_expected(
        &output_dir,
        None,
        "tests/fixtures/output_extract_multiple_lines",
    );
}
