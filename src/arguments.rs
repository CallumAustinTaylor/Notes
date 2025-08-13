use std::{env, fs::{File}, io::{Read}, path::Path};
use crate::new_note;

fn read() -> (String, String){
    let args: Vec<String> = env::args().collect();
    
    dbg!(&args);
    
    let argument = &args[1];
    let option = &args[2];

    let argument = argument.trim().to_string();
    
    let option = option.trim().to_string();
    
    (argument, option)
}

fn match_arguments(args: String, option: String, date: String) -> std::result::Result<u8, std::boxed::Box<(dyn std::error::Error + 'static)>> {
    match args.to_lowercase().as_str() {
        
        "help" => {
            println!("This is the help page.\nnote [argument] [option]\nhelp: Prints out a list of helpful information.\ndailynote: Writes a note into the ~/Documents/Notes/ directory named the day month and year.\nIf there isn't a note for that day it will create a new note.\nread: Read simply reads a note! It requires an option which is the name of the note.\nFor example, 'notes read 2025-08-13.txt' is a valid way of using read.");
            Ok(1)
        },
        
        "dailynote" => {
            let date = new_note::check_date();
            new_note::daily_note(date);
            
            Ok(2)
        },

        "read" => {
            let mut buffer = String::new();
            
            let dir = "/home/callum/Documents/Notes/";
            let path = Path::new(dir).join(format!("{option}"));
            
            let mut file = File::open(path).expect("The file you tried to open does not exist :(");            
            
            file.read_to_string(&mut buffer).expect("There were errors reading the file :(");

            println!("{buffer}");

            Ok(3)
        },
        _ => {
            println!("'{args}' is not a valid argument.");
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
