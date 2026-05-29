// References and Borrowing
// Safety and Performance
// Borrowing and references are powerful concepts

// Understanding References
// References: Enable you to borrow values without taking ownership.
// Immutable Reference.
// Mutable Reference.
// Create Reference by add "&".


// 1. Immutable reference
/*
fn main(){
    let _x: i32 = 5;
    let _y: &i32 = &_x;   //transfaring reference only.

    println!("value of x: {}", _x);
    println!("value of y: {}", _y);
}
    */
/*
//2. Mutable references.
fn main(){
    let mut _x: i32 = 5;
    let _y: &mut i32 = &mut _x;   //transfaring reference only.

    *_y += 1;
    *_y -=3;

    println!("value of x: {}", _x);
    // println!("value of y: {}", _y);  // you can have only one mutable reference and many immutable references. 
}
*/

// STRUCT:
/* 
A Data structure that allows you to group multiple fields together under one name.
*/

//===========================================
// Demonstration on one mutable reference or may immutable references
