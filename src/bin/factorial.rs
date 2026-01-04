use std::io;

fn main() {
    // Prompt the user
    println!("Enter a number to calculate factorial:");

    // Mutable string to store input
    let mut input = String::new();

    // Read input from user
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read number");

    // Parse input into an integer
    let num: i32 = match input.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Invalid number! Please enter a valid integer.");
            return; // Exit program on invalid input
        }
    };

    // Factorial for negative numbers is undefined
    if num < 0 {
        println!("Factorial is not defined for negative numbers.");
        return;
    }

    // Calculate factorial
    let mut fact: i64 = 1; // Use i64 to handle bigger factorials

    for i in 1..=num as i64 {
        fact *= i; // same as fact = fact * i
    }

    // Display result
    println!("Factorial of {} is {}", num, fact);

}
