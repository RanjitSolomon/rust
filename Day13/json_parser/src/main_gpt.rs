use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;

// Define the structure matching your JSON 
#[derive(Debug, Serialize, Deserialize)]
struct Config {
    name: String, 
    version: String, 
    enabled: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Open and read the file 
    let mut file = File::open("config.json");
    let mut data = String::new();
    file.read_to_string(&mut data);

    // Parse JSON into the struct 
    let config: Config = serde_json::from_str(&data)?;

    // Print parsed data 
    prinln!("Name: {}", config.name);
    println!("Version: {}", config.version);
    println!("Enabled: {}", config.enabled); 

    Ok({})
}