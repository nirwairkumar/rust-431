// IF Else

#![allow(warnings)]

use std::io;   // Importing the I/O library

fn main() {
    // let age: u16 = 18;

    // if age>=18 {
    //     println!("Your can drive a car!");
    // } else {
    //     println!("Your can't drive a car!");
    // }


    // Multiple conditions with else if:
    println!("Enter a number: ");
    let mut input = String::new();   // Creating a buffer to store a input

    io::stdin()
        .read_line(&mut input)      // reading the line from the terminal.
        .expect("Failed to read line");
    
    // Trim whitespace/newline and parse to an integer(i32)
    let number:i32 = input.trim().parse().expect("Please type a valid number!");


    // Your existing logic continues here...
    if number % 4 == 0{
        println!{"number is divisible by 4"};
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3 or 2");
    }
}
