use assert_cmd::Command;
use common::{run, TestResult};
use predicates::prelude::*;

const PKG: &str = "echor";

#[test]
fn dies_no_args() -> TestResult {
    let mut cmd = Command::cargo_bin(PKG)?;
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Usage"));
    Ok(())
}

#[test]
fn runs() -> TestResult {
    let mut cmd = Command::cargo_bin("echor")?;
    cmd.arg("hello").assert().success();
    Ok(())
}

#[test]
fn hello1() -> TestResult {
    let outfile = "tests/expected/hello1.txt";
    run(PKG, &["Hello there"], outfile)
}

#[test]
fn hello2() -> TestResult {
    let outfile = "tests/expected/hello2.txt";
    run(PKG, &["Hello", "there"], outfile)
}

#[test]
fn hello1_no_newline() -> TestResult {
    run(PKG, &["Hello there", "-n"], "tests/expected/hello1.n.txt")
}

#[test]
fn hello2_no_newline() -> TestResult {
    run(
        PKG,
        &["-n", "Hello", "there"],
        "tests/expected/hello2.n.txt",
    )
}
