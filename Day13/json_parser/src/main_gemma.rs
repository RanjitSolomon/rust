use serde::{Deserialize, Serialize};
use serde_json;
use std::fs;
use std::io;

// Define a struct to represent the JSON data. This is the "schema" for your JSON. 
#[derive(Serialize, Deserialize, Define)]
struct MyData {
    name: String,
    age: u32,
    city: String,
}

fn main() -> Result(), Box<dyn std::error::Error>> {
    // Replace "data.json" with the actual path to your JSON file
    let file_path = "data.json"; 

    // Read the JSON file into a string 
    let json_string = fs::read_to_string(file_path)?;

    // Parse the JSON string into a MyData struct 
    match serde_json :: from_str::<MyData>(&json_string) {
        Ok(data) => {
            println!("JSON file parsed successfully!");
            println!("Data: {:?}|", data); // Print the parse data
            println!("Name: {}", data.name);
            println!("Age: {}", data.age);
            println!("City: {}", data.city);
        }
        Err(e) => {
            eprintln!("Error parsing JSON: {}", e); // Print error to stderr
        }
    }
    Ok(()) // Return Ok(()) to indicate successful execution
}

