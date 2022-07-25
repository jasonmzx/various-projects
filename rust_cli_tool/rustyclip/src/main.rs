#![allow(unused)]

// Crate Imports:
use colored::*;
use rusqlite::{Connection, Result};
use clap::App;
use copypasta::{ClipboardContext, ClipboardProvider};

//If you're on a Linux Machine, make sure to have the xorg-dev package installed
//TO install this package on Debian/Ubuntu, preform:    sudo apt-get install xorg-dev
// (Copypasta which is a fork of rust-clipboard crate requires an installation of the x11 clipboard)

//Module Imports (Local stuff)

mod print;
mod handle;

/// Search for a pattern in a file and display the lines that contain it.
#[macro_use]
extern crate clap;
extern crate copypasta;

fn help_print() -> () {
    print::header_print("Help Menu");
}

fn main() -> Result<()> {


    //Reading the CLI Structure off a YML File, this is equivalent to the clap::App builder pattern (wrapper)
    let yaml = load_yaml!("cli_structure.yml");

    //Matches all inputted args to an object which has finder functions for various values 
    let matches = App::from_yaml(yaml).get_matches();

    //CLI Argument values:
    let action = matches.value_of("ACTION").unwrap();

    //Matching action
    match action {
        "save" => handle::save(),
        _=> handle::not_found(),
    }

    let mut ctx = ClipboardContext::new().unwrap();

    let msg : String = "Hello, world!".to_string();

    ctx.set_contents(msg).unwrap();
    println!("{:?}", ctx.get_contents().unwrap());

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