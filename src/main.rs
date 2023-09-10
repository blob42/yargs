/* TODO:
 * . add clap for CLI flags
 * . read input as column field1920s
 * . test splitting input into fields
 * . execute arbitrary shell commands to manipulate input
 * . dynamically generate field parameters ?
 */

use clap::Parser;
use yargs::parsing::DEFAULT_SEP_PATTERN;

#[derive(Parser)]
/// yargs - map commands to columns of text input
///
/// The yargs command maps commands to text columns, it works like `xargs` for tabular text. Input
/// is parsed into columns then passed to commands specified by the user. Commands are mapped to
/// specific columns using positional
/// arguments.
///
/// The first command is applied to the first column, the second command to the second column, etc.
#[command(name="yargs")]
#[command(author="blob42")]
#[command(version="0.1")]
struct Cli {
    /// separator character used to split text into columns
    #[arg(default_value_t=DEFAULT_SEP_PATTERN.to_owned())]
    #[arg(short)]
    delimiter: String,

    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    /// execute CMD each column of input. 0 < N_CMD < NB_COLUMNS
    commands: Vec<String>
}

fn main() {
    let cli = Cli::parse();

    // if let None = cli.f1.as_deref() {
    //     eprintln!("no field --fX to operate on");
    //     process::exit(1);
    // }

    if cli.debug > 0 {
        println!("{:?}", cli.delimiter);
    }

    for c in cli.commands {
        println!("- {}", c);
    }
}

