use std::{env, fs::{File}, io::{Read}, path::Path}; 
use crate::new_note;

fn read() -> (String, String){
    let args: Vec<String> = env::args().collect();
    
    let mut argument = String::new();
    let mut option = String::new();
    
    if args.len() == 2 {
        argument = args[1].trim().to_string();
    }

    if args.len() >= 3 {
        option = args[2].trim().to_string();
    }
    
    (argument, option)
}

fn match_arguments(args: String, option: String, date: String) -> std::result::Result<u8, std::boxed::Box<(dyn std::error::Error + 'static)>> {
    match args.to_lowercase().as_str() {
        "" => {
            println!("Run this program with the 'help' argument to get a list of commands.");
            
            Ok(1)
        }
        
        "help" => {
            println!("This is the help page.\nnote [argument] [option]\nhelp: Prints out a list of helpful information.\ndailynote: Writes a note into the ~/Documents/Notes/ directory named the day month and year.\nIf there isn't a note for that day it will create a new note.\nread: Read simply reads a note! It requires an option which is the name of the note.\nFor example, 'notes read 2025-08-13.txt' is a valid way of using read.");
            Ok(2)
        },
        
        "dailynote" => {
            let date = new_note::check_date();
            
            new_note::daily_note(date)?;

            Ok(3)
        },

        "read" => {
            let mut buffer = String::new();
            
            let dir = "/home/callum/Documents/Notes/";
            let path = Path::new(dir).join(format!("{option}"));
            
            let mut file = File::open(path).expect("The file you tried to open does not exist :(");            
            
            file.read_to_string(&mut buffer).expect("There were errors reading the file :(");

            println!("{buffer}");

            Ok(4)
        },
        _ => {
            println!("'{args}' is not a valid argument.\nRun this program with the 'help' argument to get a list of commands.");
            Ok(0)
        },
    }
}

pub fn arguments() -> u8 {
    let date = new_note::check_date();
    let (args, option) = read();
    let output:u8 = match_arguments(args, option, date).expect("There was an error matching arguments.");
    output
}

//When you come back to this, here are the clap docs for making arguments:
// https://docs.rs/clap/4.5.43/clap/
