struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn new(width: u32, height: u32) -> Self {
        Self{
            width,
            height,
        }

    }
}

impl Rectangle {
    fn area(&self) -> u32{

        self.width * self.height

    }
}

impl Rectangle {
    fn is_square(&self) -> bool {

        self.width == self.height
    }
}


fn main(){

   
let rect= Rectangle::new(10,10);

println!("Area = {}",rect.area());
println!("Is square = {}",rect.is_square());

println!("------------------");

let rect1= Rectangle::new(50,10);

println!("Area = {}",rect1.area());
println!("Is square = {}",rect1.is_square());


    

   

}