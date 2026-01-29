fn main() {

    // A tuple containing two i32 values
    let tup = (10, 20);

    // This tuple uses the Copy trait because:
    // - i32 is a fixed-size type
    // - Fixed-size types are stored on the stack
    // - Types stored entirely on the stack are usually Copy
    //
    // So when we pass `tup` to the function,
    // its value is COPIED, not moved.
    let sum = add(tup);

    // tup is still usable here because ownership was not moved
    println!("sum = {sum}");
}

fn add(tup: (i32, i32)) -> i32 {
    // Access tuple elements using dot notation
    // tup.0 -> first element
    // tup.1 -> second element
    tup.0 + tup.1
}
