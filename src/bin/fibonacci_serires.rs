use std::io;
fn main() {
    println!("Enter a num to generate fibonacci series");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("failed to read number");

    let num: i32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid number");
            return;
        }
    };

    cal_fibonacci(num);
}

fn cal_fibonacci(num: i32) {
    let mut first_num = 0;
    let mut second_num = 1;

    for i in 0..num {
        if i == 0 {
            println!("{first_num}");
        } 
        else if i == 1 {
            println!("{second_num}");
        } 
        else {
            
            let next = first_num + second_num;
            first_num = second_num;
            second_num = next;

            println!("{next}");
        }
    }
}
