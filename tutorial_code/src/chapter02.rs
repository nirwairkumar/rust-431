// Compound Data Types
// arrays, tuples, slices, strings (slice string)

/* Rust has two types of format: 1. Debuggable Format{:?}, 2. Display Format{}.
   For array we use Debuggable format. {:?} */

fn main(){
    let numbers: [i32; 5] = [1,2,3,4,5];
    println!("Number Array: {:?}", numbers);

    /* let mix = [1,2, "apple", true];
    println!("Mix Array: {:?}", mix); */
    // >> error

    let fruits: [&str; 3] = ["Apple", "Banana", "Orange"];
    println!("Fruits Array: {:?}", fruits);
    println!("Fruits Array 1st element: {}", fruits[0]);
    println!("Fruits Array 2nd element: {}", fruits[1]);
    println!("Fruits Array 3rd element: {}", fruits[2]);

//==========================================================
// Tuples
    let human = ("Alice", 20, false);
    println!("Human Tuple: {:?}", human);

    let person: (String, i32, bool) = ("Rohit".to_string(), 24, true); // if you will not add .to_string(). it will throw error because of slice string.
    println!("Person tuple: {:?}", person);
//>> Person tuple: ("Rohit", 24, true)
    let my_mix_tuple = ("Pravin", 23, true, [1,22,3,34,5]);
    println!("My_mixed_tuple: {:?}", my_mix_tuple);
// >> My_mixed_tuple: ("Pravin", 23, true, [1, 22, 3, 34, 5])
//===================================================

// Slices: [1,2,3,4,5] -> it will go one by one from first char.
    let number_slices :&[i32] = &[22,33,4,44,555];
    println!("Number Slice: {:?}", number_slices);
// >> Number Slice: [22, 33, 4, 44, 555]

    let animal_slices :&[&str] = &["Lion", "Elephant", "Crocodile"];
    println!("Animals: {:?}", animal_slices);

    let book_slices :&[&String] = &[&"IT".to_string(), &"Harry Potter".to_string(), &"ZEN".to_string()];
    println!("Books: {:?}", book_slices);
/*
>> Number Slice: [22, 33, 4, 44, 555]
>> Animals: ["Lion", "Elephant", "Crocodile"]
>> Books: ["IT", "Harry Potter", "ZEN"] */

//=======================================
// Strings Vs String Slices (&str)
// Strings [growable, mutable, owned string type ]

// Rust: no garbage collection and automatic memory enhancement. But slow as compare of C or C++
// Any datatype in rust is immutable. you can not change it.
// Strings are allocated in heape

    let mut stone_cold: String = String::from("Hell, ");
    println!("Stone Cold Says: {}", stone_cold); // >> Stone Cold Says: Hell, 

    stone_cold.push_str("Yeah!"); // you have to add "mut" before variable name.
    println!("Stone Cold Says: {}", stone_cold); // >> Stone Cold Says: Hell, Yeah!, 
    

// B- &str (String Slice)
    let string: String = String::from("Hello, Niwair!");
    let slice: &str = &string[0..5];  //>> Hello
    println!("Slice value: {}", slice);
    let slice_all :&str = &string;
    println!("Slice All: {}", slice_all); //>>Slice All: Hello, Niwair!

}
//=========== error============
/* variable outside function will not be accessable

fn print(){
    println!("Slice: {}", slice_all);   //>> error: slice_all not found in this scope
}
*/

//=============compiled-output==============
/*
Number Array: [1, 2, 3, 4, 5]
Fruits Array: ["Apple", "Banana", "Orange"]
Fruits Array 1st element: Apple
Fruits Array 2nd element: Banana
Fruits Array 3rd element: Orange
Human Tuple: ("Alice", 20, false)
Person tuple: ("Rohit", 24, true)
My_mixed_tuple: ("Pravin", 23, true, [1, 22, 3, 34, 5])
Number Slice: [22, 33, 4, 44, 555]
Animals: ["Lion", "Elephant", "Crocodile"]
Books: ["IT", "Harry Potter", "ZEN"]
Stone Cold Says: Hell, 
Stone Cold Says: Hell, Yeah!
Slice value: Hello
Slice All: Hello, Niwair!
*/