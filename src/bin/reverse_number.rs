use std::io;

fn main(){

     // Prompt the user
    println!("Enter a number to calculate factorial:");

    // Mutable string to store input
    let mut input = String::new();

    // Read input from user
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read number");

    // Parse input into an integer
    let mut num: i32 = match input.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Invalid number! Please enter a valid integer.");
            return; // Exit program on invalid input
        }
    };
  
    let mut reverse=0;   
  

    while num != 0{
        let digit = num % 10; // Get last digit
        reverse = reverse * 10 + digit; // APpend digit to reverse
        num = num / 10; // remove last digit

    }
    
    println!("Reverse number is {reverse}");
    
}