use assert_cmd::Command;
use std::fs;

#[test]
fn run1() {
    let expected = fs::read_to_string("tests/expected.txt").unwrap();
    Command::cargo_bin("cat")
        .unwrap()
        .arg("tests/input.txt")
        .assert()
        .success()
        .stdout(expected);
}

#[test]
fn run2() {
    let expected = fs::read_to_string("tests/expected_with_line_number.txt").unwrap();
    Command::cargo_bin("cat")
        .unwrap()
        .arg("-n")
        .arg("tests/input.txt")
        .assert()
        .success()
        .stdout(expected);
}
