use rusqlite::{Connection, Result};
use copypasta::{ClipboardContext, ClipboardProvider};

pub fn save(conn: &Connection , uuid : String) -> () {


    //Check Unique Key


    //Creation of Clipboard context: (Must be mutable since get/set fns are being applied I think ?)
    let mut ctx = ClipboardContext::new().unwrap();

    let paste_string : String = ctx.get_contents().unwrap().to_string();

    let mut stmt = conn.prepare("INSERT INTO paste_table (uuid,paste) VALUES (?,?)").unwrap();

    stmt.execute([uuid, paste_string]);

    println!("{}", "HELLO!")
}

pub fn not_found() -> () {
    println!("{}", "Not found")
}