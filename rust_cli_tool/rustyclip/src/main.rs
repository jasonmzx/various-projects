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

    //Constants:
        //amount of pastes to be displayed per page:
    const ITEMS_PER_PAGE : i32 = 5 ;


    //Assertions: 
    let mut payload_string : String = "".to_string();


    //Whilst preforming the `save` action, a user MUST include a unique indentifier for their paste.
    if( (action == "save" || action == "copy" || action == "view")  && payload == Err("")) {
        print::missing_key_panic();
        return Ok(());
    } else if ( payload != Err("") ) {
        payload_string = payload.unwrap().to_string();
    }

    let mut page_integer : i32 = -1;


    //Whilst preforming the `copy` action, a user MUST include a unique indentifier for their paste.
    if(action == "list" && payload == Err("") ) {

        page_integer = 0;

    } else {

        //Parse the integer from a string, if NaN, set to 0 

        page_integer = payload.unwrap().to_string().parse().unwrap_or(-1);
    }

    // Switch statement for Action handling (Granted that the assertions handled any invalid input)

    match action {
          "save" =>   handle::save(&conn, payload_string),
          "s" =>      handle::save(&conn, payload_string),

          "copy" =>   handle::copy(&conn, payload_string),
          "c" =>      handle::copy(&conn, payload_string),

          "view" =>   handle::view(&conn, payload_string),
          "v" =>      handle::view(&conn, payload_string),

          "delete" => handle::delete(&conn, payload_string),
          "del"    => handle::delete(&conn, payload_string),

          "list" =>   handle::list(&conn, &ITEMS_PER_PAGE, &page_integer),
          "l"    =>   handle::list(&conn, &ITEMS_PER_PAGE, &page_integer),
          
        _=> handle::not_found(),
    }

    Ok(())
}