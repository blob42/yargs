/* TODO:
 * . add clap for CLI flags
 * . read input as column field1920s
 * . test splitting input into fields
 * . execute arbitrary shell commands to manipulate input
 * . dynamically generate field parameters ?
 */

use clap::{Parser};
use std::process;

mod input;



/// # Parsing parameters
///  . parsing x-args as field parameters (-f1 'x argument')
///  . detecting number of columns and x-args from positional arguments
#[derive(Parser)]
#[command(name="colmap")]
#[command(author="blob42")]
#[command(version="0.1")]
#[command(about = "execute commands on columns")]
struct Cli {

    /// separator character used to split text into columns
    #[arg(default_value=" ")]
    #[arg(short, long = "sep")]
    separator: Option<char>,

    #[arg(long, help="select field 1")]
    f1: Option<String>,
    #[arg(long, help="select field 2")]
    f2: Option<String>,
    #[arg(long, help="select field 3")]
    f3: Option<String>,
    #[arg(long, help="select field 4")]
    f4: Option<String>,
    #[arg(long, help="select field 5")]
    f5: Option<String>,
    #[arg(long, help="select field 6")]
    f6: Option<String>,

    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,
}

fn main() {
    let cli = Cli::parse();

    // 1. parse cli parameters
    // 2. read from stdin
    // 3. split stdin into columns (column/awk commands)
    // 3. execute every field command on it's corresponding column
    //    [ ] execute a command on first text column
    // 4. print resulting concatenated columns


    if let None = cli.f1.as_deref() {
        eprintln!("no field --fX to operate on");
        process::exit(1);
    }

    if cli.debug > 0 {
        println!("{:?}", cli.separator.unwrap());
        println!("{:?}", cli.f1.unwrap());
    }


}

