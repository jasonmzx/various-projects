
use colored::*;

//Functions that print to console need to be public (pub) since they are called as a module from main.rs

pub fn header_print(title : &str) -> () {
    println!(
        "{} {} {}",
        "<-------".blue() , 
        title.bright_blue().bold() ,
        "------->".blue()
    );
}

pub fn override_print() -> () {
    println!("{}\n{}",
    "This unique identifier already exists!".bright_yellow().bold(),
    "Wanna override? ( Y / N ) :".bright_yellow().bold()
    )
}

pub fn saved_print(paste : &String , unique_keys : Vec<String> , id : i64) -> () {

    let zero_string : String = "Successfully saved paste # ".to_owned()+&id.to_string();

    let mut one_string : String = "Known identifiers: \n".to_owned();

    for key in unique_keys {
        one_string += &key.to_string();
    }

    println!("\n{}\n{}\n\n{}\n{}",
    zero_string.bright_green().bold(),
    one_string,
    "Paste contents: ".bright_green().bold(),
    paste
    )
}

//Panic prints 

pub fn save_panic() -> () {
    println!("\n{}\n{}\n{}",
    "You forgot to add a unique key to your paste!".bright_red().bold(), 
    "USAGE : rustyclip.exe save <unique key>".red(),
    "EXAMPLE: rustyclip.exe save jasons_email_address".red()
    )
}