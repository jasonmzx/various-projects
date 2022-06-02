#![allow(unused)]

// Imports:
use clap::Parser;

use rusqlite::{Connection, Result};
use rusqlite::NO_PARAMS;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The table i'm looking for
    table: String,
    /// operation
    operation: String,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    let conn = Connection::open("feather.db")?;
    println!("{}", args.table);

    conn.execute(
        "CREATE TABLE paste (
            uuid  TEXT NOT NULL,
            paste  TEXT NOT NULL
        )",
        [], // empty list of parameters.
    )?;

    Ok(())
}