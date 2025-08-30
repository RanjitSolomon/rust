# rust
## Install
sudo apt install rustc      
sudo apt install rustup       
       
### update
rustup update      
      
### Hello World 
cargo new hello_rust      
       
### Files created 
cargo.toml      
src > main.rs      
      
### Run
cd hello_rust      
cargo run       
       
### Build
##### build without running
cargo build        
cargo build --release      
      
### Check
##### check for errors without building
cargo check       
rustc --version      
cargo --version      
      
### Test
##### run test cases
cargo test      
       
### Documentation
cargo doc       
         
### Rust Playground
https://play.rust-lang.org/?version=stable&mode=debug&edition=2024      
         
         
### Day 2 - Temperature Converter
Command-Line interface (CLI) tool to convert temperatures between Celsius and Fahrenheit       
F = (C x 9/5) + 32        
C = (F - 32) x 5/9     
let mut choice = String::new();  // mutable String variable        
io::stdin().read_line(&mut choice).expect("Failed") // Read std input and assigned to variable     
let choice: u32 = match choice.trim().parse() {Ok(num) => num, Err(_) => {println!("Error); return;}}; // remove spaces, convert to integer, match integer else error.       
if, else if, else       
fn celsius_to_fahrenheit() {  // function       
let temp: f64 = match temp.trim().parse() { // convert to float       
println!("{:.2}°F is {:.2}°C", temp, celsius);  // format output       
       
### Day 3 - Simple Calculator
Tokenize the input      
// tokenize the input. input (5 + 3) contains 3 items: "5" and "+" and "3"     
let tokens: Vec<&str> = input.trim().split_whitespace().collect();      
let num1: f64 = match tokens[0].parse() {Ok(n) => n,Err(_) => {println!("❌ Invalid first number.");
return;}};     
let operator = tokens[1];     
    let result = match operator {"+" => add(num1, num2),"-" => subtract(num1, num2),"*" => multiply(num1, num2),
        "/" => divide(num1, num2), _ => {
            println!("❌ Invalid operator. Use +, -, *, or /.");
            return;
        }
    };       

### Day 4 - Gussing Game
Random number generation, user input, and control flow        
Under [dependenies], add the following line to "cargo.toml" and then "cargo build"      
rand = "0.8" //  Current version is 0.8.4 as at Aug 2025      
use rand::Rng; // Generate Random number      
use std::cmp::Ordering // compare numbers if greater or lesser     
let secret_number = rand::thread_rng().gen_range(1..=100);      
loop{ continue, break}      

### Day 5 - Word Count
Count the number of words in a text file     
File input/output standard library, String manipulation      
Command Line Arguments             
      
### Day 6 - BMI Calculator
Body Mass Index       
Function: Return optional value - fn get_input_as_f64() -> Option<f64> {        
Function: Takes two args and retun         
fn calculate_bmi(weight: f64, height: f64) -> f64 {        
Function: return static string      
fn classify_bmi(bmi: f64) -> &'static str {       
        
### Day 7 - Palindrome Checker
Sequence of letters or numbers that can be read forward or backwards.      
racecar      
A man, a plan, a canal, Panama      
input        
        .chars() // Iterate over each character        
        .filter(|c| c.is_alphanumeric()) // Keep only letters and numbers        
        .map(|c| c.to_lowercase().to_string()) // Convert to lowercase        
        .collect::<String>() // Collect into a new String         
         
### Day 8 - Fibonacci Sequence Generator
Fibonacci sequence up to a user-defined number of terms      
Each number is sum of previous two numbers       
F(n) = F(n-1) + F(n-2)         
First 10 numbers: 0,1,1,2,3,5,8,13,21,34        
Validating user input      
For Loop: for i in 2..n {          
           
### Day 9 - Prime Number checker.
Check if a given number is prime.      
Prime number is a natural number greater than 1 and divisible by 1 and itself.     
2,3,5,7,11          

### Day 10 - To-Do List App 
Add, View, Mark as complete, delte tasks     
Add dependencies to cargo.toml        
serde = { version="1.0", features=["derive"]}       
serde_json = "1.0"       
serialization, deserialization    
cargo build        

### Day 11 - Basic Timer Tool 
Set a timer for a specified duration and notify       
- Enter a duration (in hours, minutes, or seconds)      
- Start the timer and display a countdown      
- Notify when the time is up       
         
### Day 12 - Rock-Paper-Scissors Game
cargo.toml [dependencies] rand="0.8"      
- Display a welcome message and instructions       
- User chooses Rock, Paper, or Scissors        
- The computer randomly selects a choice      
- Compare choices to determine the winner      

