
//-----------------UTF8--------------
fn main(){

//1
let s: String = "whatever".to_string();
// 2
let s: String = String :: from("whatever");
// Mutate the variable [push to it]
let mut s: String = String :: from("foo");
s.push_str("bar");
s.push('!');


println!("the value of s = {}", s);


let salam: String = String::from("هتاف للترحيب");
let salut: String = String::from("Salut");

// If you want to combine strings, use the + operator
let s1: String = String::from("Hello, ");
let s2: String = String::from("world!");
let s3: String = s1 + &s2; // note s1 has been moved here and can no longer be used


println!("the value of s3 {}", s3);

// Formatting Strings
let full_message: String = format!("{} {}", salam, salut);
println!("{full_message}");


}