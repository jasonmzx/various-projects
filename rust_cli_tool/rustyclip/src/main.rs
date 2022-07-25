#![allow(unused)]

// Crate Imports:
use colored::*;
use rusqlite::{Connection, Result};

//Module Imports (Local stuff)

mod print;

/// Search for a pattern in a file and display the lines that contain it.
#[macro_use]
extern crate clap;
use clap::App;

fn help_print() -> () {
    print::header_print("Help Menu");
}

fn main() -> Result<()> {

    let yaml = load_yaml!("cli_structure.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let input_match = matches.value_of("INPUT").unwrap();

    println!("{}", input_match);


    //Connection to SQLLite
    let conn = Connection::open("feather.db")?;
    // println!("{}", args.operation);

    // if(args.operation == "help"){
    //     help_print();
    //     return Ok(());
    // }


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