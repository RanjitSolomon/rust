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
       


