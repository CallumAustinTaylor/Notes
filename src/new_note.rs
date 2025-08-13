use chrono::prelude::*;
use std::{path::Path, io::{self, Write}, fs::{File, OpenOptions}};

// In order to create a note file, I need to check the date and time. I can do this with chrono. 
// I'll start by making a mod that is called makenote. Inside that, I'll add more mods that are
// needed to do the job.

pub fn check_date () -> String {
    let date: DateTime<Local> = Local::now();
    date.format("%Y-%m-%d").to_string()
}

fn create_note (date: &String) -> std::io::Result<()> {
    let dir = "/home/callum/Documents/Notes/";
    let path = Path::new(dir).join(format!("{date}.txt"));
    File::create(path)?;
    Ok(())
}

pub fn make_note() -> Result<(), Box<dyn std::error::Error>> {
    let date: String = check_date();

    create_note(&date)?;

    println!("{date} successfully created!");
    
    Ok(())
}

// Everything for arguments go below here:

pub fn daily_note(date: String) {
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
}
