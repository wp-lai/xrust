use assert_cmd::Command;

#[test]
fn dies_no_arg() {
    let mut cmd = Command::cargo_bin("echo").unwrap();
    cmd.assert().failure();
}

#[test]
fn runs() {
    let expected = "hello world!\n";
    let mut cmd = Command::cargo_bin("echo").unwrap();
    cmd.arg("hello world!").assert().success().stdout(expected);
}
