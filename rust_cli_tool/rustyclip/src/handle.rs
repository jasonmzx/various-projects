use rusqlite::{Connection, Result};
use copypasta::{ClipboardContext, ClipboardProvider};

pub fn save(conn: &Connection, ctx : &ClipboardContext, uuid : String) -> () {


    let mut stmt = conn.prepare("INSERT INTO paste (uuid,paste) VALUES (?,?)").unwrap();

    stmt.execute([uuid,"paste".to_string()]);

    println!("{}", "HELLO!")
}

pub fn not_found() -> () {
    println!("{}", "Not found")
}