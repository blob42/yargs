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
fn no_stdin_no_args(){
    let mut cmd = Command::cargo_bin("yargs").unwrap();
    let assert = cmd
        .write_stdin("")
        .assert();
    assert.stdout("");
}

// input with many columns
// no positional arguments
// behaves like cat
#[test]
fn stdin_no_args() -> TestResult {
    let input = Path::new("tests/inputs/input1");

    let mut cmd = Command::cargo_bin("yargs").unwrap();
    let assert = cmd
        .pipe_stdin(input)?
        .assert();
    assert.stdout(read_to_string(input)?);
    Ok(())
}


// n args <= n cols
#[test]
fn cli_pass2()  {
    let mut cmd = Command::cargo_bin("yargs").unwrap();
    cmd.args(["-d", r"\s+"])
    .args(vec!["true"; 6]);
    run_command("tests/inputs/input1", &mut cmd).unwrap()
        .success();
}

// should fail if more yargs provided than detected columns
// the input text has 7 columns, we pass 8 yargs
// delimiter: space
#[test]
fn cli_fail1() {
    let mut cmd = Command::cargo_bin("yargs").unwrap();
    cmd.args(["-d", r"\s+"])
    .args(vec!["arg"; 8]);
    run_command("tests/inputs/input1", &mut cmd).unwrap()
        .failure();
}

// should execute yarg corresponding to column
// first yarg transforms col to uppercase
// second yarg reverses the text 
#[test]
fn cli_exec_yarg() {
    let mut cmd = Command::cargo_bin("yargs").unwrap();
    cmd.args(["-d", r"\s+"])
    .args(["tr '[:lower:]' '[:upper:]'", "rev"]);
    run_command("tests/inputs/input2_spaces", &mut cmd).unwrap()
        .success()
        .stdout(read_to_string(Path::new("tests/outputs/output2_spaces")).unwrap());

}
