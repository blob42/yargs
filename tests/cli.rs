use std::error::Error;
use assert_cmd::Command;
use assert_cmd::assert::Assert;
// use assert_cmd::prelude::*;
use std::path::Path;
use std::fs::read_to_string;

type TestResult = Result<(), Box<dyn Error>>;
type AssertResult = Result<Assert, Box<dyn Error>>;


// pub fn args<I, S>(&mut self, args: I) -> &mut Self
// where
//     I: IntoIterator<Item = S>,
//     S: AsRef<ffi::OsStr>,
// ────────────────────────────────────────────────
fn run_command(test_file: &str, cmd: &mut Command) -> AssertResult 
{
    let input = Path::new(test_file);
    Ok(cmd.pipe_stdin(input)?.assert())
}

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

// n args <= n cols
#[test]
fn cli_pass2()  {
    let mut cmd = Command::cargo_bin("yargs").unwrap();
    cmd.args(&["-d", r"\s"])
    .args(&["1", "2", "3", "4", "5", "6"]);
    run_command("tests/inputs/input1", &mut cmd).unwrap()
        .success();
}

#[test]
#[should_panic]
// more arguments passed than columns
// delimiter: space
fn cli_fail1() {
    let mut cmd = Command::cargo_bin("yargs").unwrap();
    cmd.args(&["-d", r"\s"])
    .args(&["1", "2", "3", "4", "5", "6", "7", "8"]);
    run_command("tests/inputs/input1", &mut cmd).unwrap()
        .failure();
}
