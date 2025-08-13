use chrono::prelude::*;
use std::{env, fs::{File, OpenOptions}, io::{self, Read, Write}, path::Path};

fn read() -> (String, String){
    let args: Vec<String> = env::args().collect();
    
    dbg!(&args);
    
    let argument = &args[1];
    let option = &args[2];

    let argument = argument.trim().to_string();
    
    let option = option.trim().to_string();
    
    (argument, option)
}

fn check_date () -> String {
    
    let date: DateTime<Local> = Local::now();
    
    date.format("%Y-%m-%d").to_string()
}

fn match_arguments(args: String, option: String, date: String) -> std::result::Result<u8, std::boxed::Box<(dyn std::error::Error + 'static)>> {
    match args.to_lowercase().as_str() {
        
        "help" => {
            println!("This is the help page.");
            Ok(1)
        },
        
        "dailynote" => {
            println!("What would you like to write?");
            let mut buf = String::new();
            io::stdin().read_line(&mut buf).expect("There was an issue reading your input for dailynote.");
            let dir = "/home/callum/Documents/Notes/";
            let path = Path::new(dir).join(format!("{date}.txt"));
            let mut file = OpenOptions::new()
                .create(true)   // create if it doesn't exist
                .write(true)    // open for writing
                .append(true)   // or .truncate(true) for overwriting
                .open(path)?;
            if !buf.ends_with('\n') {
                buf.push('\n');
            }
            
            file.write_all(buf.as_bytes())?;
            
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
    let date = check_date();
    let (args, option) = read();
    let output:u8 = match_arguments(args, option, date).expect("There was an error matching arguments.");
    output
}

//When you come back to this, here are the clap docs for making arguments:
// https://docs.rs/clap/4.5.43/clap/
