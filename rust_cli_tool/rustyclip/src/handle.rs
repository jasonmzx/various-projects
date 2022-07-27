use rusqlite::{Connection, Result};

pub fn save(conn: &Connection) -> () {
    let mut stmt = conn.prepare("INSERT INTO paste (uuid,paste) VALUES (?,?)").unwrap();

    stmt.execute(["test","paste"]);

    println!("{}", "HELLO!")
}

pub fn not_found() -> () {
    println!("{}", "Not found")
}