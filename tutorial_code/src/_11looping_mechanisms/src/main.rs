// # repetition with loops:
// # Doing things over and over
// loop
// while
// for

fn main() {
    // Loop keyword -> run untill and unless you manually stop it. 

    // loop{   
    //     println!("Hello, world!");
    // }

//--------------------------------------------

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 20 {
            break counter - 100;
        };
    };
    println!("The new result is {result}");

//--------------------------------------------
    let mut count = 0;
    'counting_up: loop {
        println!("count:{count}");
        let mut remaining = 10;
        loop{
            println!("remaining: {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -=1;
            count +=1;
        }
    }

// while loop;
    let mut number = 3;
    while number !=0 {
        println!("{number}");
        number -=1;
        
    }
    println!("HEY!!!!");

// # Looping through a collection with for loop
    let a = [1,2,3,4,4,5,6];
    for element in a {
        println!("{element}");
    }

}
