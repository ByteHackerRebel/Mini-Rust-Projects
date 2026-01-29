struct Point(i32,i32);

impl Point{
    fn distance_from_origin(&self) -> i32 {

        self.0.abs() + self.1.abs()   

    }
}
fn main(){

    let distance = Point(10,20);    
    let dis = Point(60,70);
    
    println!("Distance on point 10,20 is = {}", distance.distance_from_origin());
    println!("Distance on point 60,70 is = {}", dis.distance_from_origin());
    

}
