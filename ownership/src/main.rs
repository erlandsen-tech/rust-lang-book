mod second;
use second::change;
use second::takes_and_gives;

fn main() {
    let mut s = String::from("hello");
    s.push_str(", world!");

    println!("{}", s);

    for i in s.chars() {
        println!("{}", i);
    }

    let s2 = s.clone();
    println!("{}", takes_and_gives(s2));

    let x = 5;
    let mut y = x;
    y = y + 1;
    println!("");
    println!("{}", y);
    println!("{}", x);

    let mut s4 = String::from("hello");
    change(&mut s4);
    println!("{}", s4);
}
