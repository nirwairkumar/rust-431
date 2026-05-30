// Shadowing
// Shadowing is not the same as marking a variable as mutable.
fn main() {
    let X = 5;  // result is 5
    
    let X = X + 1; // result is 6

    let X = X + 2; // result is 8

    {
        let X = X * 2;
        println!("The value of X in the inner scope is: {}", X);
        //>> result is 16
    }
    println!("The value of X in main function is: {X}"); // result is 8
}
