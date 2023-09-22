//TODO:

use std::error::Error;
use assert_cmd::Command;
// use assert_cmd::prelude::*;
use std::path::Path;
use std::fs::read_to_string;

type TestResult = Result<(), Box<dyn Error>>;

#[test]
fn pass(){
    let mut cmd = Command::cargo_bin("yargs").unwrap();
    cmd.assert().success();
}



// input with many columns
// no positional arguments
// behaves like cat
#[test]
fn pass_noargs() -> TestResult {
    let input = Path::new("tests/inputs/input1");

    let mut cmd = Command::cargo_bin("yargs").unwrap();
    let assert = cmd
        .pipe_stdin(input)?
        .assert();
    assert.stdout(read_to_string(input)?);
    Ok(())
}

