use chrono::prelude::*;
use std::{path::Path, fs::File};

// In order to create a note file, I need to check the date and time. I can do this with chrono. 
// I'll start by making a mod that is called makenote. Inside that, I'll add more mods that are
// needed to do the job.

fn check_date () -> String {
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
