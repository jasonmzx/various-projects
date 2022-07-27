
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

pub fn save_panic() -> () {
    println!("{}\n{}\n{}",
    "You forgot to add a unique key to your paste!".bright_red().bold(), 
    "USAGE : rustyclip.exe save <uuid>".red(),
    "EXAMPLE: rustyclip.exe save jasons_email_address".red()
    )
}