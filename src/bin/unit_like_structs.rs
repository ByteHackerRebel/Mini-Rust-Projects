
struct Logger;

impl Logger{
    fn log(&self){        
        println!("Logging...");
    }
}
fn main(){

    let unit=Logger;

    unit.log();


}