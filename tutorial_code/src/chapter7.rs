//constants
//it is also immmutable but you can not convert into mutable by adding mut at the begning!!!

fn main(){
    println!("Hello, world!");
    let mut x = 5;

//  const mut Y:i32 = 10;   //cannot be mutable
    const Y:i32 = 10;    // should have an upper case name
    println!("the value of x is {}", x);
    println!("the value of y is {}", Y);

    println!("the value of pi is {}", PI);

    println!("Three hours in seconds: {}", THREE_HOURS_IN_SECONDS);  //Three hours in seconds: 10800
}

// Your can declare a constant with a type annotation irrespective of any scope as globle scope.

const PI:f64 = 3.141592653;  //it will be accerssable in any scope.
const THREE_HOURS_IN_SECONDS:u32 = 60 * 60 * 3;

//>> output----
// Hello, world!
// the value of x is 5
// the value of y is 10
// the value of pi is 3.141592653
// Three hours in seconds: 10800