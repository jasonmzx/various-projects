use rusqlite::{params, Connection, Result};
use copypasta::{ClipboardContext, ClipboardProvider};

//Module imports
mod handle_print;

//Query Map struct

#[derive(Debug)]
struct KeyTable {
    id : i32,
    key : String
}


fn execute_paste(conn : &Connection, paste_string : &String) -> (i64) {

    //This function executes the Insert query by passing in the clipboard's current content
    //Then returns the last inserted row's id (i64 integer) to be used elsewhere

    let mut stmt = conn.prepare("INSERT INTO paste_table (paste) VALUES (?)").unwrap();

    stmt.execute([paste_string]);

    conn.last_insert_rowid()

}

pub fn save(conn: &Connection , key : String) -> () {
    
    //Creation of Clipboard context: (Must be mutable since get/set fns are being applied I think ?)

    let mut ctx = ClipboardContext::new().unwrap();

    let paste_string : String = ctx.get_contents().unwrap().to_string();


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
           
            //Pass in pointers since we're only trying to borrow these values in the execution of paste

            let row_id : i64 = execute_paste(&conn, &paste_string);

            let mut update_stmt = conn.prepare("UPDATE key_table SET id = ?1 WHERE key = ?2;").unwrap();

            update_stmt.execute(params![row_id, key]);

            //Printing the successful feedback to the user:

            let keys : Vec<String> = [key].to_vec(); 

            handle_print::saved_print(&paste_string, keys, row_id );

            return ();
        }

        println!("{:?}" , "I'll take that as a No...");
        return ();
        
    }

    // Unique Key wasn't used, proceed as normal:

    let row_id : i64 = execute_paste(&conn, &paste_string);

    let mut insert_stmt = conn.prepare("INSERT INTO key_table (id , key) VALUES (?1 , ?2);").unwrap();

    insert_stmt.execute(params![row_id, key]);

    let keys : Vec<String> = [key].to_vec(); 

    handle_print::saved_print(&paste_string, keys, row_id );


    //let mut stmt = conn.prepare("INSERT INTO paste_table (uuid,paste) VALUES (?,?)").unwrap();

   // stmt.execute([uuid, paste_string]);

    println!("{}", "HELLO!")
}

pub fn not_found() -> () {
    println!("{}", "Not found")
}