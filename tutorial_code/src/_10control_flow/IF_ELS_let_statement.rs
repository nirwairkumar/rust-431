// Using if in a let statement

use std::io;

fn main(){
    println!("Type true or false: ");
    let mut input = String::new();
    
io::stdin()
    .read_line(&mut input)
    .expect("Failed to read the line");
let condition: bool = input.trim().parse().expect("please try a valid boolean(true/false)");

    // let condition = true;
    let number = if condition {5} else {6};
    println!("Number: {number}");
}

