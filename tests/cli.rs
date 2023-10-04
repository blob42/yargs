//TODO:

use std::error::Error;
use assert_cmd::Command;
// use assert_cmd::prelude::*;
use std::path::Path;
use std::fs::read_to_string;

type TestResult = Result<(), Box<dyn Error>>;

// empty stdin should return an empty line
#[test]
fn pass(){
    let mut cmd = Command::cargo_bin("yargs").unwrap();
    let assert = cmd
        .write_stdin("")
        .assert();
    assert.stdout("");
}



#[test]
// input with many columns
// no positional arguments
// behaves like cat
fn pass_columns_no_args() -> TestResult {
    let input = Path::new("tests/inputs/input1");

    let mut cmd = Command::cargo_bin("yargs").unwrap();
    let assert = cmd
        .pipe_stdin(input)?
        .assert();
    assert.stdout(read_to_string(input)?);
    Ok(())
}


#[test]
// should if more yargs provided than detected columns
fn fail_yargs_mismatch1() -> TestResult {
    let input = Path::new("tests/inputs/input1");

    let mut cmd = Command::cargo_bin("yargs").unwrap();

    let assert = cmd
        .args(&["one", "two"])
        .pipe_stdin(input)?
        .assert();
    assert.failure();
    Ok(())
}


