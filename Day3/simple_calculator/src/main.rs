use std::io;

fn main() {
    println!("🧮 Simple Calculator");
    println!("Available operations: +, -, *, /");
    println!("Enter your expression (e.g., 5 + 3):");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read input");

    // tokenize the input. input contains 3 items: "5" and "+" and "3"
    let tokens: Vec<&str> = input.trim().split_whitespace().collect();

    if tokens.len() != 3 {
        println!("❌ Invalid input. Please follow the format: number operator number");
        return;
    }

    let num1: f64 = match tokens[0].parse() {
        Ok(n) => n,
        Err(_) => {
            println!("❌ Invalid first number.");
            return;
        }
    };

    let num2: f64 = match tokens[2].parse() {
        Ok(n) => n,
        Err(_) => {
            println!("❌ Invalid second number.");
            return;
        }
    }; 

    let operator = tokens[1];
    let result = match operator {
        "+" => add(num1, num2),
        "-" => subtract(num1, num2),
        "*" => multiply(num1, num2),
        "/" => divide(num1, num2),
        _ => {
            println!("❌ Invalid operator. Use +, -, *, or /.");
            return;
        }
    };

    println!("✅ Result: {:.2}", result);
}

fn add(a: f64, b: f64) -> f64 {
    a + b
}

fn subtract(a: f64, b: f64) -> f64 {
    a - b
}

fn multiply(a: f64, b: f64) -> f64 {
    a * b
}

fn divide(a: f64, b: f64) -> f64 {
    if b == 0.0 {
        println!("❌ Division by zero is not allowed.");
        std::process::exit(1);
    }
    a / b
}

