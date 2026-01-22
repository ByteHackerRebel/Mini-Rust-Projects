use std::io;

fn main(){

    println!("1 to convert temperature Fahrenheit into Celcius");
    println!("2 to convert temperature Celcius into Fahrenheit");

    let mut input= String::new();

    io::stdin()
    .read_line(&mut input)
    .expect("Failed to read number");

    let num:i32 =match input.trim().parse(){
        Ok(num)=>num,
        Err(_)=>{
            println!("Invalid number");
            return;
        }
    };


    match num {
        1 => fahrenheit_to_celsius(),
        2 => celsius_to_fahrenheit(),
        _ => {
            println!("Invalid number");
            return;
        }
    };


}

fn fahrenheit_to_celsius(){}


fn celsius_to_fahrenheit(){

    println!("Enter temperature in Celsius");

    let mut celsius = String::new();

    io::stdin()
    .read_line(&mut celsius)
    .expect("Failed to read number");

    let celsius: f64 = match celsius.trim().parse(){
        Ok(n) => n,
        Err(_) => {
            println!("Invalid number");
            return;
        }
    };

    let fahrenheit = (celsius * 9.0 / 5.0) + 32.0;

    println!("Fahrenheit = {fahrenheit}");
}