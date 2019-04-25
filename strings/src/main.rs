fn main() {
    let mut s = String::new();
    s = "litt tekst ".to_string();
    s.push_str("mere");
    println!("{}", s);

    let s2 = String::from("United states of ");
    let s3 = s2 + &"whatever".to_string();
    println!("{}", s3);

    //format
    let s4 = String::from("one");
    let s5 = String::from("two");
    let s6 = String::from("three");
    let s= format!("{}-\\/-{}-\\/-{}\\/", s4, s5, s6);
    println!("{}", s);
}
