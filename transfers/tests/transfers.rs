// Copyright (C) 2017 Kisio Digital and/or its affiliates.
//
// This program is free software: you can redistribute it and/or modify it
// under the terms of the GNU Affero General Public License as published by the
// Free Software Foundation, version 3.

// This program is distributed in the hope that it will be useful, but WITHOUT
// ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS
// FOR A PARTICULAR PURPOSE. See the GNU Affero General Public License for more
// details.

// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>

use assert_cmd::prelude::*;
use std::{path::Path, process::Command};
use tempfile::TempDir;
use transfers::{transfers, TransfersMode};
use transit_model::test_utils::*;

#[test]
fn test_generates_all_transfers() {
    test_in_tmp_dir(|path| {
        let input_dir = "./tests/fixtures/input";
        let model = transit_model::ntfs::read(input_dir).unwrap();
        let rules: Vec<Box<Path>> = vec![];

        let model = transfers(model, 100.0, 0.785, 120, &TransfersMode::All, rules, None).unwrap();

        transit_model::ntfs::write(&model, path, get_test_datetime()).unwrap();
        compare_output_dir_with_expected(
            &path,
            Some(vec!["transfers.txt"]),
            "./tests/fixtures/output_all",
        );
    });
}

#[test]
fn test_generates_transfers_intra_contributors() {
    test_in_tmp_dir(|path| {
        let input_dir = "./tests/fixtures/input";
        let model = transit_model::ntfs::read(input_dir).unwrap();
        let rules: Vec<Box<Path>> = vec![];

        let model = transfers(
            model,
            100.0,
            0.785,
            120,
            &TransfersMode::IntraContributor,
            rules,
            None,
        )
        .unwrap();

        transit_model::ntfs::write(&model, path, get_test_datetime()).unwrap();
        compare_output_dir_with_expected(
            &path,
            Some(vec!["transfers.txt"]),
            "./tests/fixtures/output_intra_contributors",
        );
    });
}

#[test]
fn test_generates_transfers_inter_contributors() {
    test_in_tmp_dir(|path| {
        let input_dir = "./tests/fixtures/input";
        let model = transit_model::ntfs::read(input_dir).unwrap();
        let rules: Vec<Box<Path>> = vec![];

        let model = transfers(
            model,
            100.0,
            0.785,
            120,
            &TransfersMode::InterContributor,
            rules,
            None,
        )
        .unwrap();

        transit_model::ntfs::write(&model, path, get_test_datetime()).unwrap();
        compare_output_dir_with_expected(
            &path,
            Some(vec!["transfers.txt"]),
            "./tests/fixtures/output_inter_contributors",
        );
    });
}

#[test]
fn test_generates_transfers_with_modification_rules() {
    test_in_tmp_dir(|path| {
        let input_dir = "tests/fixtures/input";
        let model = transit_model::ntfs::read(input_dir).unwrap();
        let rules = vec![Path::new("./tests/fixtures/rules.txt").to_path_buf()];
        let report_path = path.join("report.json");

        let model = transfers(
            model,
            100.0,
            0.785,
            120,
            &TransfersMode::All,
            rules,
            Some(report_path),
        )
        .unwrap();

        transit_model::ntfs::write(&model, path, get_test_datetime()).unwrap();
        compare_output_dir_with_expected(
            &path,
            Some(vec!["transfers.txt", "report.json"]),
            "./tests/fixtures/output_rules",
        );
    });
}

#[test]
fn test_bainary_generates_all_transfers_with_rules() {
    let output_dir = TempDir::new().expect("create temp dir failed");
    let report_path = output_dir.path().join("report.json");
    Command::cargo_bin("transfers")
        .expect("Failed to find binary 'transfers'")
        .arg("--input")
        .arg("tests/fixtures/input/")
        .arg("--max-distance")
        .arg("100.0")
        .arg("--walking-speed")
        .arg("0.785")
        .arg("--waiting-time")
        .arg("120")
        .arg("--rules-file")
        .arg("tests/fixtures/rules.txt")
        .arg("--report")
        .arg(report_path.to_str().unwrap())
        .arg("--output")
        .arg(output_dir.path().to_str().unwrap())
        .assert()
        .success();
    compare_output_dir_with_expected(
        &output_dir,
        Some(vec!["transfers.txt", "report.json"]),
        "tests/fixtures/output_all_with_rules",
    );
}