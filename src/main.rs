/* TODO:
 * . add clap for CLI flags
 * . read input as column field1920s
 * . test splitting input into fields
 * . execute arbitrary shell commands to manipulate input
 * . dynamically generate field parameters ?
 */

#![allow(unused_imports)]
use clap::{Parser,CommandFactory};
use clap::error::ErrorKind;
use yargs::{DEFAULT_SEP_PATTERN, stdin};
use yargs::parse::InputText;
use anyhow::Result;
use std::io::{BufRead, Read, BufReader, stdin};
use std::process::{self, Command, Stdio};


#[derive(Parser)]
/// yargs - map commands to columns of text input
///
/// The yargs command maps commands to text columns, it works like `xargs` for tabular text. Input
/// is parsed into columns then passed to commands specified by the user. Commands are mapped to
/// specific columns using positional
/// arguments.
///
/// The first command is applied to the first column, the second command to the second column, etc.
#[derive(Debug)]
#[command(name="yargs")]
#[command(author="blob42")]
#[command(version="0.1")]
struct Cli {
    /// Regex used to  to split input into columns
    #[arg(default_value=DEFAULT_SEP_PATTERN)]
    #[arg(short)]
    delimiter: String,

    //TODO:
    // -f --field
    // skip fields with `-`

    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,

    /// execute CMD each column of input. 0 < N_CMD < NB_COLUMNS
    yargs: Vec<String>
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // if let None = cli.f1.as_deref() {
    //     eprintln!("no field --fX to operate on");
    //     process::exit(1);
    // }


    if cli.verbose > 0 {
        eprintln!("======\nDEBUG:\n");
        eprintln!("{:?}", cli);

        for cmd in &cli.yargs {
            println!("- {}", cmd);
        }
    }


    // input validation
    // take input text, split_columns, nb yargs <= nb columns
    // Validate that the number of positional args <= nb of text columns
    // ex: input: hello foo bar
    // --
    // possible ways to call the app:
    // $ echo 'hello foo bar' | yargs cat rev 'tr -d b'
    // $ echo 'hello foo bar' | yargs cat rev
    // $ echo 'hello foo bar' | yargs cat
    // let mut cmd = Cli::command();

    // Read commands as positional args

    // Read input from stdin
    let raw_input = stdin::read_stdin()?;
    let input_text = InputText::new(&raw_input, &cli.delimiter);

    let n_cols = match input_text.n_cols() {
        Err(e) => {
            eprintln!("error parsing input: {}", e);
            process::exit(1)
        },
        Ok(n) => n,
    };

    // Check that n args <= input cols
    if cli.yargs.len() > input_text.n_cols()? {
        // panic!("too many arguments");
        eprint!("too many arguments for delimiter={:?}", input_text.sep);
        process::exit(1);
    }

    if cli.verbose > 0 {
        eprintln!("detected {n_cols} colunms");
        eprintln!("======");
    }

    // TODO: RESULT
    if cli.yargs.is_empty() {
        print!("{}", raw_input);
    } else {
        // Handle yargs
        // Exec each yarg as a process for each column
        // yarg #1 for column 1, yarg #2 -> col 2 ... 

        // we know we have at least one elm
        let yarg = cli.yargs.first().unwrap();
        let yarg_cmd = Command::new(yarg)
            .stdout(Stdio::piped())
            .spawn()
            .expect(&format!("Failed to exec {yarg}"));
    }


    Ok(())
}
