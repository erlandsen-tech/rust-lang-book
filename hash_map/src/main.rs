use std::collections::HashMap;
fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 90);

    let teams = vec![String::from("Purple"), String::from("Orange")];
    let initial_scores = vec![99, 12];
    let scores: HashMap<_,_> = teams.iter().zip(initial_scores.iter()).collect();
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
}
