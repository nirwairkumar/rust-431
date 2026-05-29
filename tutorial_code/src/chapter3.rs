//rust always find the main function and expect all other functions inside the main function.
/*-----error---not-in-main-function--------
fn hello(){
    println!("Hello, world!!!");
}
*/
//an function / variables should be written in snake case(all letters in small and separated by underscore).
// **snake case: person_one
// kabab case: person-two
//======================================

fn main(){
    hello_world();     //Hello, Rust!!
    tell_height(165);  //My height is 165 cm.
    human_id("Rohan",23, 170.3);  //My name is Rohan, I am 23 years old, and my height is 170.3 cm.
    human_id("Preet", 25, 165.0); //My name is Preet, I am 25 years old, and my height is 165 cm.

//  --------last-line-automatically-assign-as-returen-in-expression--------------------------------
    let _x: i32 ={
        let price :i32 = 5;
        let qty: i32 = 10;
        price * qty          // automatically assigned as return;
    };
    println!("Total Cost: {}", _x);   //>> Total Cost: 50

    let y: i32 = add(34,6);
    println!("34 + 6 = {}", y);  //34 + 6 = 40

    println!("The value of 'add' function is: {}", add(4,6) );  //>>The value of 'add' function is: 10

    // Calling the BMI function
    let weight: f64 = 70.0;
    let height: f64 = 1.82;
    let bmi: f64 = calculate_bmi(weight,height);
    println!("Your BMI is: {:.2}", bmi);    //Your BMI is: 21.13

}

fn hello_world(){
    println!("Hello, Rust!!");
}
// we can insert input values
fn tell_height(height: u32){
    println!("My height is {} cm.", height);
}

//we can insert more thatn one value
fn human_id(name: &str, age:u32, height:f32){
    println!("My name is {}, I am {} years old, and my height is {} cm.",name, age, height);
}

//=============================================
// Expressions: Anything that returns a value.
//       eg. 5, true & false, add(3,4), if condition {value1} else {value2}.
// Statements: Anything that does not return a value.
 
// functions returning values
fn add(a:i32, b:i32) -> i32{
    a+b
}


// Final Example: BMI
// BMI = weight(kg)/height(m)^2

fn calculate_bmi(weight_kg:f64, height_m:f64) -> f64{
    weight_kg / (height_m*height_m)
}
