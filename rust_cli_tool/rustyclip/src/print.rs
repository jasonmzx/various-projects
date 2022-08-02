
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

//Panic prints 

pub fn missing_key_panic() -> () {
    println!("\n{}\n{}\n{}",
    "You forgot to add a unique key to your paste!".bright_red().bold(), 
    "USAGE : rustyclip.exe <command> <unique key>".red(),
    "EXAMPLE: rustyclip.exe <command> jasons_email_address".red()
    )
}
