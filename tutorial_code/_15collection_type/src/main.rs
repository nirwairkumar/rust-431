// Collection Types
// Vectors  Vec<T>


fn main(){

/*
    let _v:Vec<i32> = Vec::new();

    // Macro to create a vector of numbers
    
    let mut _v: Vec<i32> = Vec::new();
    let mut _v: Vec<i32> = vec![1,2,3];

//     let mut _the_numbers_ver:Vec<i32> = Vec::new();

    _v.push(5);
    _v.push(6);
    _v.push(7);
    _v.push(8);
    _v.push(9);

    println!("{:?}", _v);

*/

    let _v: Vec<i32> = vec![1,2,3,4,5];

    // let third: &i32 = &_v[2];  // Direct indexing

    // println!("The third element is {third}");

    let third: Option<&i32> = _v.get(index: 2);
    match third {
        Some(third: &i32) => println!("the third element for a GET method is {third}"),
        None => println!("there is no third element."),
    }



}