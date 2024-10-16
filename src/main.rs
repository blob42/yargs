/* TODO:
 * . add clap for CLI flags
 * . read input as column field1920s
 * . test splitting input into fields
 * . execute arbitrary shell commands to manipulate input
 * . dynamically generate field parameters ?
 */

#![allow(unused_imports)]
use anyhow::{anyhow, Result};
use clap::error::ErrorKind;
use clap::{CommandFactory, Parser};
use log::debug;
use std::io::Write;
use std::process::{self, Command, Stdio};
use std::{io, os};
use yargs::parse::{split_columns, InputText};
use yargs::{stdin, Column, Columns, DEFAULT_SEP_PATTERN};

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
#[command(name = "yargs")]
#[command(author = "blob42")]
#[command(version = "0.1")]
struct Cli {
    /// Regex used for splitting the input into columns
    #[arg(default_value=DEFAULT_SEP_PATTERN)]
    #[arg(short)]
    delimiter: String,

    /// Run yarg cmd for each row element
    #[arg(short)]
    xarg: Option<String>,

    //TODO:
    // -f --field
    // skip fields with `-`
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,

    /// execute CMD each column of input. 0 < N_CMD < NB_COLUMNS
    yargs: Vec<String>,
}

// NOTE: BUFFERED MODE
fn main() -> Result<()> {
    let cli = Cli::parse();
    let mut logger = env_logger::builder();

    // if let None = cli.f1.as_deref() {
    //     eprintln!("no field --fX to operate on");
    //     process::exit(1);
    // }

    if cli.verbose >= 3 {
        eprintln!("======\nDEBUG:\n");
        eprintln!("{:#?}", cli);
        for cmd in &cli.yargs {
            println!("- {}", cmd);
        }
        eprintln!("======");
    }
    if std::env::var("RUST_LOG").is_err() {
        match cli.verbose {
            0 => {
                logger.filter_level(log::LevelFilter::Warn);
            }
            1 => {
                logger.filter_level(log::LevelFilter::Info);
            }
            2 => {
                logger.filter_level(log::LevelFilter::Debug);
            }
            3 => {
                logger.filter_level(log::LevelFilter::Trace);
            }
            _ => {}
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
    let input_text = InputText::new(raw_input.as_ref(), &cli.delimiter);

    let n_cols = match input_text.n_cols() {
        Err(e) => {
            eprintln!("error parsing input: {}", e);
            process::exit(1)
        }
        Ok(n) => n,
    };

    // Check that n args <= input cols
    if cli.yargs.len() > input_text.n_cols()? {
        // panic!("too many arguments");
        eprint!("too many arguments for delimiter={:?}", input_text.sep);
        process::exit(1);
    }

    debug!("detected {n_cols} colunms");
    debug!("======");

    let columns = input_text.split()?;

    // output columns with same input shape and capacity
    let mut out_cols: Vec<Vec<String>> = Vec::with_capacity(columns.len());

    if cli.yargs.is_empty() {
        print!("{}", raw_input);
    } else {
        // Handle yargs
        // For each column Y, execute the Yth arg command on column

        // naive implementation
        // i is the number of columns
        for (i, yarg) in cli.yargs.into_iter().enumerate() {
            //out_cols.resize(i+1, vec![]);

            if yarg == "-" {
                continue;
            }

            // let yarg_cmd = &yarg.split_whitespace().nth(0).unwrap_or(&yarg);
            // dbg!(yarg_cmd);
            // let yarg_args: Vec<&str> = yarg.split_whitespace().skip(1).collect();
            // dbg!(yarg_args);

            // for each yarg split into cmd, args
            let mut yarg_cmd = Command::new(yarg.split_whitespace().next().unwrap_or(&yarg))
                .args(yarg.split_whitespace().skip(1))
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
                .unwrap_or_else(|_| panic!("Failed to exec {yarg}"));

            let mut yarg_stdin = yarg_cmd.stdin.take().expect("failed to open stdin");
            let _columns = columns.clone();
            // yarg_stdin.write_all(columns[i].join("\n").as_bytes())?;

            // TODO!: use https://crates.io/crates/subprocess to spawn shell
            // expl: using a long `awk ..` will fail if args are split on space the cmd needs to be
            // passed as is to a subshell
            std::thread::spawn(move || {
                //dbg!(&_columns[i]);
                yarg_stdin
                    .write_all(_columns[i].join("\n").as_bytes())
                    .expect("Failed to write to stdin");
            });

            // gather output from child
            let output = yarg_cmd
                .wait_with_output()
                .unwrap_or_else(|_| panic!("failed to read stdout for {}", yarg));

            //TODO!: check child stderr
            if !output.stderr.is_empty() {
                eprintln!(
                    "executing yarg command: `{}`:\n{}",
                    yarg,
                    String::from_utf8_lossy(&output.stderr)
                );
                process::exit(1)
            }

            // println!("{}", String::from_utf8_lossy(&output.stdout));

            // output should be a \n separated column of text
            let out_col: Column = String::from_utf8_lossy(&output.stdout)
                .split("\n")
                .map(String::from)
                .collect();

            out_cols.insert(i, out_col);
        }

        let out_cols: Columns = out_cols.try_into()?;

        println!("{}", out_cols);
        process::exit(0);

        // DEBUG:
        // we know we have at least one elm
        let yarg = cli.yargs.first().unwrap();
        // execute the child process (yarg) using the
        // piped input

        // EXAMPLE: using thread to spawn the processes
        // std::thread::spawn(move || {
        //     stdin.write_all("Hello, world!".as_bytes()).expect("Failed to write to stdin");
        // });
        let mut yarg_cmd = Command::new(yarg)
            .stdin(Stdio::piped())
            .spawn()
            .unwrap_or_else(|_| panic!("Failed to exec {yarg}"));

        let mut yarg_stdin = yarg_cmd.stdin.take().expect("failed to open stdin");
        yarg_stdin.write_all(columns[5].join("\n").as_bytes())?;

        // println!("{}", String::from_utf8(output.stdout).unwrap());
    }

    Ok(())
}
