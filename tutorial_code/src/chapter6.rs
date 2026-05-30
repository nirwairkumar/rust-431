//Variables & Mutability

//variables are immutable in rust.

/*
fn main(){
    let _a: i32 = 5;
    println!("The value of a is {}", _a);
    _a = 10;      // this is compilation error.<^^^^^^^ cannot assign twice to immutable variable>
    println!("The new value of a is {}", _a);
}
*/


//----successfull run after adding mut----------
fn main(){
    let mut _a: i32 = 5;
    println!("The value of a is {}", _a);
    _a = 10;      // this is compilation error.<^^^^^^^ cannot assign twice to immutable variable>
    println!("The new value of a is {}", _a);
}
    