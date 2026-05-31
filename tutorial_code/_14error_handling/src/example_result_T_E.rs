


fn divide_result(numerator: f64, denominator: f64) -> Result<f64, String>{
    if denominator == 0.0 {
        Err("Cant divide by 0".to_string())
    } else {
        Ok(numerator / denominator)
    }
}

fn main(){
    match divide_result(100.23, 34.4){
        Ok(result) => println!("Result:{}", result),
        Err(err) => println!("Error: {}", err),
    }
}