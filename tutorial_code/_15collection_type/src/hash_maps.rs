fn main(){

// --------Hashmaps-----------
    let mut scores: HashMap<String, i32> = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name: String = String::from("Blue");
    let score: i32 = score.get(&team_name).copied().unwrap.or(default:0);

    for (key: &String, value: &i32) in &scores {
        println!("{key}: {value}");
    }


}