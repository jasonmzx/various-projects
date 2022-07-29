use rusqlite::{params, Connection, Result};
use copypasta::{ClipboardContext, ClipboardProvider};

//Query Map struct

#[derive(Debug)]
struct KeyTable {
    id : i32,
    key : String
}

fn execute_paste(conn : &Connection) -> (i64) {

    //This function executes the Insert query by passing in the clipboard's current content
    //Then returns the last inserted row's id (i64 integer) to be used elsewhere

    let mut stmt = conn.prepare("INSERT INTO paste_table (paste) VALUES (?)").unwrap();

    let mut ctx = ClipboardContext::new().unwrap();

    let paste_string : String = ctx.get_contents().unwrap().to_string();

    stmt.execute([paste_string]);

    conn.last_insert_rowid()

}

pub fn save(conn: &Connection , key : String) -> () {

    //Check Unique Key
    let mut check_stmt = conn.prepare("SELECT id , key FROM key_table WHERE key = :key;").unwrap();

    let rows = check_stmt.query_map(&[(":key", key.as_str())], |row| {
        Ok(KeyTable {
            id: row.get(0)?,
            key: row.get(1)?,
        })
    }).unwrap();

    //If Unique Key exists, give user the option to override the nickname with a new paste

    if(rows.count() >= 1){
        println!("{}", "This unique identifier already exists!");
        //Get an Extra input from the user

        let mut line = String::new();
        println!("Wanna override? ( Y / N ) :");
        std::io::stdin().read_line(&mut line).unwrap();

        //Trim removes whitespace at the end and casts String to &str 

        let l_str : &str = line.trim(); //I know the type is inferred but this is for readibility

        if(l_str == "Y" || l_str == "y" ){
           
            let row_id : i64 = execute_paste(&conn);

            let mut update_stmt = conn.prepare("UPDATE key_table SET id = ?1 WHERE key = ?2;").unwrap();

            update_stmt.execute(params![row_id, key]);

            //TODO: Make this print prettier
            println!("{:?}", row_id);

            return ();
        }

        println!("{:?}" , "I'll take that as a No...");
        return ();
        
    }


    //Creation of Clipboard context: (Must be mutable since get/set fns are being applied I think ?)
    let mut ctx = ClipboardContext::new().unwrap();

    let paste_string : String = ctx.get_contents().unwrap().to_string();

    //let mut stmt = conn.prepare("INSERT INTO paste_table (uuid,paste) VALUES (?,?)").unwrap();

   // stmt.execute([uuid, paste_string]);

    println!("{}", "HELLO!")
}

pub fn not_found() -> () {
    println!("{}", "Not found")
}