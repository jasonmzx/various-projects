use rusqlite::{params, Connection, Result};
use copypasta::{ClipboardContext, ClipboardProvider};
use std::io;
use std::io::{Error, ErrorKind};

//Module imports
mod handle_print;

//Query Map struct

#[derive(Debug)]
struct Table {
    id : i32,
    payload : String
}

//TODO: Make a copy function that executes the copy, and voids if it passes but raises an error if fail
fn execute_io_copy(string : String) -> Result<() , Error> {

    //Creation clipboard context
    let mut ctx = ClipboardContext::new().unwrap();

    //Before transferring ownership of string, I'm cloning it to check it later
    let string_clone = string.clone();

    ctx.set_contents(string).unwrap();

    let check = ctx.get_contents();
    
    if(check.unwrap() != string_clone) {
        let msg = format!("Invalid array size!");
        return Err(Error::new(ErrorKind::InvalidData, msg));
    }

    Ok(())

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
        Ok(Table {
            id: row.get(0)?,
            payload: row.get(1)?,
        })
    }).unwrap();

    //If Unique Key exists, give user the option to override the nickname with a new paste

    if(rows.count() >= 1){
        handle_print::override_print();
        //Get an Extra input from the user

        let mut line = String::new();
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

}

pub fn copy(conn: &Connection , key : String) -> () {

    handle_print::copy_print(&key);

    //Creation clipboard context
    let mut ctx = ClipboardContext::new().unwrap();

    //Grab paste from identifier: Using 2 queries

    let mut key_stmt = conn.prepare("SELECT id , key FROM key_table WHERE key = :key;").unwrap();
    
    let key_rows = key_stmt.query_map(&[(":key", key.as_str())], |row| {
        Ok(Table {
            id: row.get(0)?,
            payload: row.get(1)?,
        })
    }).unwrap();

    //Putting in a default value since the compiler is worried.
    let mut reference_id : i32 = -1;

    //For loop is nessessary since MappedRow type cannot be indexed regularly (weird)
    for row in key_rows {

        //SQLite library assumes integers as i32 so I convert it back into i64
        reference_id = row.unwrap().id;

    }

    //ASSERT: Check if the key actually exists before trying section query
    if(&reference_id == &-1 ){
        handle_print::not_existent_key_error();
        return ();
    }
    
    //Get paste via Id

    let mut paste_stmt = conn.prepare("SELECT id , paste FROM paste_table WHERE id = :key;").unwrap();

    let paste_rows = paste_stmt.query_map(&[(":key", reference_id.to_string().as_str())], |row| {
        Ok(Table {
            id: row.get(0)?,
            payload: row.get(1)?,
        })
    }).unwrap();

    let mut copy_string : String = "".to_string();

    //Grab payload from query's row struct
    for row in paste_rows {
        copy_string = row.unwrap().payload;
    }


    //Execution of the Copy into your Clipboard: 

    match execute_io_copy(copy_string) 
    {
        Ok(i) => {
            handle_print::copy_success(); //If the copy returns Ok, print the success statement
        },

        Err(e) => {println!("I'm not ok...")}, //If it returns an error, let the user know this function failed
    };


    return ();
}

pub fn view(conn: &Connection , key : String) -> () {

}


//TODO Make use of page integer being passed in, (including the default value)
pub fn list(conn: &Connection, items_per_page : &i32, page : &i32) -> () {

    let mut all_paste_stmt = conn.prepare("SELECT * FROM paste_table;").unwrap();

    //Page bounds:
    let lower_bound : i32 = items_per_page * page;
    let upper_bound : i32 = items_per_page * ( page + 1 );


    println!("LB : {} UB : {}", lower_bound.to_string(), upper_bound.to_string());
    //query

    let all_paste_rows = all_paste_stmt.query_map([], |row| {
        Ok(Table {
            id: row.get(0)?,
            payload: row.get(1)?,
        })
    }).unwrap();

    let mut item_counter : i32 = 0 ;

    for paste_row in all_paste_rows {


        if(item_counter >= lower_bound && item_counter <= upper_bound) {

            //Unwrap and move the paste_row's contents to a variable 
            let uw_paste_row = paste_row.unwrap();

            //Set variables to pointers of various struct elements for easy calling below
            let reference_id : &i32 = &uw_paste_row.id;
            let paste : &String = &uw_paste_row.payload;

            let mut cor_stmt = conn.prepare("SELECT id,key FROM key_table WHERE id = :id;").unwrap();

            let corrolated_rows = cor_stmt.query_map(&[(":id", reference_id.to_string().as_str())], |row| {
                Ok(Table {
                    id: row.get(0)?,
                    payload: row.get(1)?,
                })
            }).unwrap();

            let mut keys : Vec<String> = Vec::new();

            //Double nest for to get all associated nicknames from a reference id:
            for cor_row in corrolated_rows {
                
                //Key string
                let nickname = cor_row.unwrap().payload;

                keys.push(nickname);

            }

            let joined_keys = keys.join(" , ");

            handle_print::view_print(&reference_id, &paste , &joined_keys);

        }

        item_counter += 1;
    }
}

pub fn delete(conn: &Connection, key: String) -> () {
    
    handle_print::delete_print(&key);

    let mut key_stmt = conn.prepare("SELECT id , key FROM key_table WHERE key = :key;").unwrap();
    
    let key_rows = key_stmt.query_map(&[(":key", key.as_str())], |row| {
        Ok(Table {
            id: row.get(0)?,
            payload: row.get(1)?,
        })
    }).unwrap();

    //Putting in a default value since the compiler is worried.
    let mut reference_id : i32 = -1;

    //For loop is nessessary since MappedRow type cannot be indexed regularly (weird)
    for row in key_rows {

        //SQLite library assumes integers as i32 so I convert it back into i64
        reference_id = row.unwrap().id;

    }

    //ASSERT: Check if the key actually exists before trying section query
    if(&reference_id == &-1 ){
            handle_print::not_existent_key_error();
            return ();
    }


    //TODO: Possibly turn this into a transaction instead of having 2 seperate executions via the rusqlite transaction method

    //Delete all tied nicknames

    let mut delete_keys_stmt = conn.prepare("DELETE FROM key_table WHERE id = ?1 ;").unwrap();

    delete_keys_stmt.execute(params![&reference_id]);
    

    //Delete all tied pastes

    let mut delete_paste_stmt = conn.prepare("DELETE FROM paste_table WHERE id = ?1 ;").unwrap();

    delete_paste_stmt.execute(params![&reference_id]);

    handle_print::delete_success();
    
}

pub fn not_found() -> () {
    println!("{}", "Not found")
}