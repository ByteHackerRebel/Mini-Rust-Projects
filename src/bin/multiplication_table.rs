use std::io;

fn main() {
    // Prompt the user to enter a number
    println!("Enter a number:");

    // Create a mutable String to store user input
    let mut input = String::new();

    // Read input from standard input (keyboard)
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read number");

    // Try to convert the input into an integer
    let num: i32 = match input.trim().parse() {
        Ok(num) => num, // Successfully parsed number
        Err(_) => {
            // Handle invalid input
            println!("Invalid number! Please enter a valid integer.");
            return; // Exit the program
        }
    };

    cal_mul(num);
}

fn cal_mul(num: i32) {
    // Print multiplication table from 1 to 10
    for i in 1..=10 {
        // {} is a placeholder; {num} and {i} are Rust 1.58+ "named" format specifiers
        println!("{num} * {i} = {}", num * i);
    }
}
