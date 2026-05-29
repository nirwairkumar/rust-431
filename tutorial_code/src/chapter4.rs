// Ownership, Borrowing and References

// Ownership
//----------
// C, C++ -> Memory Management Control Issue
// Garbage Collector solved this issue, but created a new issue -> Slow Performence:
// [Stop the program to remove garbages and then Resume]
// Rust is for memory safety

// OWNERSHIP introduced by Rust to solve memory safety issues and high performance at the same time.
// What is Ownership?
// Every value has a single owner [every variable has one value, and it is its sole owner].

// Ownership Rules
//----------------
// 1. Each value in Rust has a variable that's its owner.
// 2. There can be only one owner at a time.
// 3. When the owner goes out of scope, the value will be dropped.

/*
// ---- 1st rule's example-----
fn main(){
    let s1 = String::from("zomato");

    let len = calculate_length(&s1);  // passing refrance of owner(s1), now ownership to s in calculate_length function.
    println!("Length of '{}' is {}", s1, len); //>>Length of 'zomato' is 6
}

fn calculate_length(s: &String)-> usize{
    s.len()
}
*/

/*
//------2nd rule's example-----------
fn main(){
    let s2 = String::from("flipkart");
    let s3 = s2;        // ownership has transfred and no longer to access.

    //println!("{}", s2);   //>>error _<^^ value borrowed here after move>_
    println!("{}", s3);    // >> flipkart
}
*/


//-----3rd rule's example----------
fn main(){
    let s4 = String::from("Amazon");
    let len = calculate_length(&s4);
    println!("Length of '{}' is {}.", s4, len);
} // s4 goes outside of scope and its value will be dropped

fn calculate_length(s:&String)->usize{
    s.len()
}
/*
// error-------------
fn printLost(s:&String){
    println!("{}", &s4);   //cannot find value `s4` in this scope
}*/

