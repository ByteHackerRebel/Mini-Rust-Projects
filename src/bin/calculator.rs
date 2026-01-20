use std::io;

fn main() {

    println!("Enter first number:");

    let mut input1 = String::new();

    io::stdin().
    read_line(&mut input1)
    .unwrap();

    let num1: f64 = input1
    .trim()
    .parse()
    .unwrap();

    println!("Enter second number:");

    let mut input2 = String::new();

    io::stdin()
    .read_line(&mut input2)
    .unwrap();

    let num2: f64 = input2
    .trim()
    .parse()
    .unwrap();

    println!("Choose operation (+, -, *, /):");

    let mut op = String::new();

    io::stdin()
    .read_line(&mut op)
    .unwrap();

    let op = op.trim();

    let result = match op {
        "+" => num1 + num2,
        "-" => num1 - num2,
        "*" => num1 * num2,
        "/" => {
            if num2 != 0.0 {
                num1 / num2
            } else {
                println!("Error: Cannot divide by zero.");
                return;
            }
        }
        _ => {
            println!("Invalid operator.");
            return;
        }
    };

    println!("Result: {}", result);
}
