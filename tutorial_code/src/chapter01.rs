// Primitive data types(scaler data types)
// int, float, bool, char

//Integer
//Rust has signed (+ and -) and unsigned integer (only+) types of different sizes.
// i8, i16, i32, i64, i128: Signed integers.
// u8, u16, u32, u64, u128: Unsigned inegers.

fn main(){
    let x: i32 = -42; //can hold + or -
    let y: u64 = 100; // only holdes +. if -ve then error
    println!("Signed Integer: {}", x);
    println!("Unsigned Integer:{}",y);

// i32 - range is `-2147483648..=2147483647`,
// i64 -  range is `-9223372036854775808 to 9223372036854775807`
    let e: i32 = 2147483647;
    let i: i64 = 9223372036854775807;
    println!("Maximum value of i32: {}", e);
    println!("Maximum value of i64: {}", i);
    
// ==========================================
// Floats [Floating Point Types]
// f32, f64
    let pi: f64 = 3.14;
    println!{"Value of pi {}", pi};

// Boolean Values: true, false
    let is_snowing: bool = true;
    println!{"Is it snowing ? {}", is_snowing};

// Charachter Type - char
    let letter: char = 'a';
    println!("First letter of the alphabet: {}", letter);

}


/* compiled output

Signed Integer: -42
Unsigned Integer:100
Maximum value of i32: 2147483647
Maximum value of i64: 9223372036854775807
Value of pi 3.14
Is it snowing ? true
First letter of the alphabet: a

*/