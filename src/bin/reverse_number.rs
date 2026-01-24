use std::io;

fn main(){

     // Prompt the user
    println!("Enter a number to calculate reverse:");

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
  
    rev_num(num);
    
}


fn rev_num(num: i32){

        let mut reverse=0;     
        let mut o_num=num;

    while o_num != 0{
        let digit = o_num % 10; // Get last digit
        reverse = reverse * 10 + digit; // APpend digit to reverse
        o_num = o_num / 10; // remove last digit

    }
    
    println!("Reverse number is {reverse}");
}