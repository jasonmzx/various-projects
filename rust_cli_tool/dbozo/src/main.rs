#![allow(unused)]

use clap::Parser;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The table i'm looking for
    table: String,
    /// operation
    operation: String,
}

fn main() {
    let args = Cli::parse();
    println!("{}", args.table);
}