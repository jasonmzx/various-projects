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

    //Connection to SQLLite
    let conn = Connection::open("feather.db")?;

    //Initializing Tables:

    let tables = &[        
        "CREATE TABLE paste_table (
        id  INTEGER PRIMARY KEY NOT NULL,
        paste  TEXT NOT NULL
        )"
        , 
        "CREATE TABLE key_table (
            id INTEGER NOT NULL,
            key TEXT NOT NULL,
            FOREIGN KEY (id) REFERENCES paste_table(id)
        )"
    ];
 
    //Creation of Tables: (Loop thru Vector & execute)

    for table in tables {

        match conn.execute( table , [] , ) // empty list of parameters.
        {
            Ok(i) => {
                let ok_string : String = "Initialized a new SQLite Table.".to_string();
                println!("{}", ok_string.green().bold() )
            },
    
            Err(e) => {},
        };
    };

    //Reading the CLI Structure off a YML File, this is equivalent to the clap::App builder pattern (wrapper)
    let yaml = load_yaml!("cli_structure.yml");

    //Matches all inputted args to an object which has finder functions for various values 
    let matches = App::from_yaml(yaml).get_matches();

    //CLI Argument values:
    let action = matches.value_of("ACTION").unwrap(); //Unwrapping this since the Action flag is mandatory
    let payload = matches.value_of("PAYLOAD").ok_or(""); //This flag is option so I'm equally expecting an Error 

    //Assertions: 

    //Whilst preforming the `save` action, a user MUST include a unique indentifier for their paste.
    if( (action == "save" || action == "copy" || action == "view")  && payload == Err("")) {
        print::missing_key_panic();
        return Ok(());
    }

    let mut page_integer : i32 = 0;


    //Whilst preforming the `copy` action, a user MUST include a unique indentifier for their paste.
    if(action == "list" && payload == Err("") ) {

        print::missing_key_panic();
        return Ok(());
    } else {

        //Parse the integer from a string, if NaN, set to 0 


        //TODO :: Fix this since payload_string now definied under
        page_integer = payload_string.parse().unwrap_or(0);
    }

    //Unwrapping and casting the Cli args for easy passing to the switch statement
    let payload_string : String = payload.unwrap().to_string();

    // Switch statement for Action handling (Granted that the assertions handled any invalid input)

    match action {
          "save" => handle::save(&conn, payload_string),
          "copy" => handle::copy(&conn, payload_string),
          "view" => handle::view(&conn, payload_string),
          "delete" => handle::delete(&conn, payload_string),
          "list" => handle::list(&conn, page_integer),
        _=> handle::not_found(),
    }

    Ok(())
}