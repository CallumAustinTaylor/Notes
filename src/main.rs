mod new_note;
mod arguments;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    arguments::arguments();
    Ok(())    
}
