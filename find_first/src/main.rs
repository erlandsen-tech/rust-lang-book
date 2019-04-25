use std::io;
fn main() {
    println!("Tast en streng.");
    let mut streng = String::new();
    io::stdin()
        .read_line(&mut streng)
        .expect("Skjedde visst en feil gitt.");

    if streng.trim().is_empty() {
        println!("Strengen er tom");
        std::process::exit;
    }
    let fw = first_word(&streng);
    println!("{}", fw);
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}
