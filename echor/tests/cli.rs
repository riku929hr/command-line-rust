use core::str;
use std::fs;

use anyhow::Result;
use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn dies_no_args() -> Result<()> {
    let mut cmd = Command::cargo_bin("echor")?;
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Usage"));
    Ok(())
}

fn runs(args: &[&str], expected_file: &str) -> Result<()> {
    let expected = fs::read_to_string(expected_file)?;
    let output = Command::cargo_bin("echor")?
        .args(args)
        .output()
        .expect("fail");

    let stdout = String::from_utf8(output.stdout).expect("invalid utf-8");

    assert_eq!(stdout, expected);

    Ok(())
}

// test the output is equal to Unix echo command
#[test]
fn hello1() -> Result<()> {
    runs(&["Hello there"], "tests/expected/hello1.txt")
}

#[test]
fn hello2() -> Result<()> {
    runs(&["Hello", "there"], "tests/expected/hello2.txt")
}

#[test]
fn hello1_no_newline() -> Result<()> {
    runs(&["-n", "Hello  there"], "tests/expected/hello1.n.txt")
}

#[test]
fn hello2_no_newline() -> Result<()> {
    runs(&["-n", "Hello", "there"], "tests/expected/hello2.n.txt")
}
