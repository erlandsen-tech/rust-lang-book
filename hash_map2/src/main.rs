use std::collections::HashMap;
fn main() {
    let text = "jungle jungle boogey woogey oh yea";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    for (key, value) in &map {
        println!("{}: {}", key, value);
    }
}
