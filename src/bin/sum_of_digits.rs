use std::io;

fn main(){
     println!("Enter a number");
    let mut num = String::new();

    io::stdin()
        .read_line(&mut num)
        .expect("failed to read number");

    let num: i32 = num.trim().parse().expect("failed to read number");

    let x = sum_fun(num);
    println!("Total digits are {} ", x);
}

fn sum_fun(mut num:i32)->i32{
    let mut sum=0;
    while num>0{
        let digit=num%10;
        sum=sum+digit;
        num=num/10;
    }
    sum

}