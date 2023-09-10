/* TODO:
 * . add clap for CLI flags
 * . read input as column field1920s
 * . test splitting input into fields
 * . execute arbitrary shell commands to manipulate input
 * . dynamically generate field parameters ?
 */
mod input;

use clap::Parser;
use input::DEFAULT_SEP_PATTERN;

#[derive(Parser)]
/// colmap - map commands to columns of text input
///
/// The colmap command reads text from stdin as columns. Each column is then passed to a command
/// specified by the user. Commands are mapped to specific columns using positional arguments.
///
/// The first command is applied to the first column, the second command to the second column, etc.
#[command(name="colmap")]
#[command(author="blob42")]
#[command(version="0.1")]
struct Cli {
    /// separator character used to split text into columns
    #[arg(default_value_t=DEFAULT_SEP_PATTERN.to_owned())]
    #[arg(short, long = "sep")]
    separator: String,

    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    /// execute CMD each column of input. 0 < N_CMD < NB_COLUMNS
    commands: Vec<String>
}

fn main() {
    let cli = Cli::parse();

    // 1. parse cli parameters
    // 2. read from stdin
    // 3. split stdin into columns (column/awk commands)
    // 3. execute every field command on it's corresponding column
    //    [ ] execute a command on first text column
    // 4. print resulting concatenated columns

    // if let None = cli.f1.as_deref() {
    //     eprintln!("no field --fX to operate on");
    //     process::exit(1);
    // }

    if cli.debug > 0 {
        println!("{:?}", cli.separator);
    }

    for c in cli.commands {
        println!("- {}", c);
    }
}

