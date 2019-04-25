fn main() {
    for i in 0..50 {
        if i % 3 == 0 && i % 5 == 0 {
            println!("fizzbuzz");
        }
        if i % 5 == 0 {
            println!("fizz");
        } else if i % 3 == 0 {
            println!("buzz");
        }
    }
}
