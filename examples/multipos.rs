// Using multiple positional arguments
// Detect dynamically how many positional arguments where passed and handle them
use std::path::PathBuf;

use clap::{Parser};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {

    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    /// CMD to execute for each column of input text. 0 < N_CMD < NB_COLUMNS
    #[arg( last = true )]
    commands: Vec<String>

}

fn main() {
    let cli = Cli::parse();

    // You can check the value provided by positional arguments, or option arguments
    // if let Some(name) = cli.name.as_deref() {
    //     println!("Value for name: {name}")
    // }

    if let Some(config_path) = cli.config.as_deref() {
        println!("Value for config: {}", config_path.display());
    }

    // You can see how many times a particular flag or argument occurred
    // Note, only flags can have multiple occurrences
    match cli.debug {
        0 => println!("Debug mode is off"),
        1 => println!("Debug mode is kind of on"),
        2 => println!("Debug mode is on"),
        3 | 4 | 5 => println!("too much dude !"),
        _ => println!("Don't be crazy"),
    }

    
    for c in cli.commands {
        println!("{:?}", c);
    }

    // Continued program logic goes here...
}
