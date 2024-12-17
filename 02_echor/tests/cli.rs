use anyhow::Result;
use assert_cmd::Command;
use predicates::prelude::*;
use pretty_assertions::assert_eq;

#[test]
fn dies_no_args() -> Result<()> {
    let mut cmd = Command::cargo_bin("echor")?;
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Usage"));
    Ok(())
}

#[test]
fn hello1() -> Result<()> {
    run_cmd(&["Hello there"], "tests/expected/hello1.txt")
}

#[test]
fn hello2() -> Result<()> {
    run_cmd(&["Hello", "there"], "tests/expected/hello2.txt")
}

#[test]
fn hello1_no_newline() -> Result<()> {
    run_cmd(&["Hello  there", "-n"], "tests/expected/hello1.n.txt")
}

#[test]
fn hello2_no_newline() -> Result<()> {
    run_cmd(&["Hello", "there", "-n"], "tests/expected/hello2.n.txt")
}

fn run_cmd(args: &[&str], expected_file: &str) -> Result<()> {
    let expected = std::fs::read_to_string(expected_file)?;
    let mut cmd = Command::cargo_bin("echor")?;
    let output = cmd.args(args).output().expect("failed to execute process");
    let stdout = String::from_utf8(output.stdout).expect("stdout was not valid utf8");
    assert_eq!(stdout, expected);
    Ok(())
}
