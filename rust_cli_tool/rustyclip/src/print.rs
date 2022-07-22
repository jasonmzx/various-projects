
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