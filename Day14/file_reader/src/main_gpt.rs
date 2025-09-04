use std::fs;
use std::io::{self, BufRead}; 
use std::path::Path; 

// Optional: Show line numbers 
pub fn read_file_with_line_numbers(filepath: &str) -> Result<(), io::Error> {
    let path = Path::new(filepath);
    let file = fs::File::open(path)?;
    let reader = io::BufReader::new(file); 

    for (line_number, line) in reader.lines().enumerate() {
        let line = line?; 
        println!("{}: {}", line_number + 1, line); // Line numbers start from 1
    }
    Ok(())
}

// Optional: Search for keywords 
pub fn search_keywords(filepath: &str, keywords: &[&str]) -> Result<(), io::Error>{
    let path = Path::new(filepath);
    let file = fs::File::open(path)?;
    let reader = io::BufReader::new(file);

    for (line_number, line) in reader.lines().enumerate() {
        let line = line?; 
        for keyword in keywords {
            if line.contains(keyword) {
                println!("Line {}: Keyword found: {}", line_number + 1, keyword);
            }
        }
    }
    Ok(())    
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get the file path from the user
    println!("Enter the file path:");
    let mut file_path = String::new(); 
    io::stdin().read_line(&mut file_path)?;
    let file_path = file_path.trim(); 

    // Read the file and print its contents
    println!("Reading file...");
    read_file_with_line_numbers(file_path)?; // Keywords to search for 

    println!("\nPerforming keyword search...");
    let keywords = vec!["example", "function", "Rust"]; // Keywords to search for 
    search_keywords(file_path, &keywords)?; 

    Ok(())
}

// Code Explanation
// * **Error Handling:** Uses `Result<(), io::Error>` for proper error handling.      
// The `?` operator is used to propagate errors up the call stack, making code cleaner      
// * **`BufReader`:** for efficient file reading. This improves performance, especially for large files      
// * **Line Numbers:** The `read_file_line_numbers` function iterates through the lines of the file and prints
// the line number along with the line content. Line numbers starts from 1
// * **Keyword Search:** The `search_keywords` function takes a slice of keywords and searches for them within each     
// line of the file. It prints the line number and the keyword if a match is found. 
// * **Clear Output:** The code prints informative messages to the console to indicate what it's doing. 
// * **User Input:** Prompts the user for the file path. 
// * **`Path` type:** Uses `std::path::Path` for more robust file path handling 
// * **Modularity:** The code is divided into separate functions for reading the file, shosing line numbers, and 
// searching for keywords. This makes the code more modular and easier to maintain. 
// * **`Box<dyn std::error::Error::Error>`:** Uses as the error type in the `main` function's    
// return type. this allows the function to return any type of error that implements the `Error` trait. 
// * **Comments:** Includes comments to explain the purpose of different sections of the code. 

// rustc file_reader.rs     
// run ./file_reader  - propmpts for file path

// To add more keywords, modify the `keywords` vector in the `main` function.   
// Modify the `read_file_with_line_number` and `search_keywords` functions to add more features, 
// such as filtering the output or highlighting the keywords. 