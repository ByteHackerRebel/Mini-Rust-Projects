use std::io;

fn main() {
    // Prompt the user to enter a number
    println!("Enter a number:");

    // Create a mutable String to store user input
    let mut input = String::new();

    // Read input from standard input (keyboard)
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    // Try to convert the input into an integer
    let number: i32 = match input.trim().parse() {
        Ok(num) => num,          // Successfully parsed number
        Err(_) => {
            // Handle invalid input
            println!("Invalid number! Please enter a valid integer.");
            return;
        }
    };

    // Check if the number is even or odd
    if number % 2 == 0 {
        println!("{} is Even", number);
    } else {
        println!("{} is Odd", number);
    }
}
