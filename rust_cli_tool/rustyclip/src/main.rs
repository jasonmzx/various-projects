#![allow(unused)]

// Crate Imports:
use clap::Parser;
use colored::*;
use rusqlite::{Connection, Result};

//Module Imports (Local stuff)

mod print;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]


struct Cli {
    /// The table i'm looking for
    operation: String,
    /// operation
    paste: String,
}

fn help_print() -> () {
    print::header_print("Help Menu");
}

fn main() -> Result<()> {
    let args = Cli::parse();
    let conn = Connection::open("feather.db")?;
    println!("{}", args.operation);

    if(args.operation == "help"){
        help_print();
        return Ok(());
    }


    println!("{}", "Initializing SQLLite Table:".bright_green());
    match conn.execute(
        "CREATE TABLE paste (
            uuid  TEXT NOT NULL,
            paste  TEXT NOT NULL
        )",
        [], // empty list of parameters.
    ) {
        Ok(i) => {
            let ok_string : String = i.to_string();
            println!("{}\n", ok_string.bright_green() )
        },

        Err(e) => {
            //Explicitly casting error to string
            let error_string : String = e.to_string();
            eprintln!("{}\n", error_string.bright_red() )
        },
    };


    Ok(())
}