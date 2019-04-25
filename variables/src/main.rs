use rand;
fn main() {
    let c = 'z';
    let z = 'ℤ';
    let heart_cat = '😻';
    println!("c {}", c);
    println!("z {}", z);
    println!("heart_cat {}", heart_cat);
    let tup = (500, 234.1, "test");
    println!("The value of y is: {} ", tup.1);
    println!("The value of x is: {} ", tup.0);
    println!("The value of z is: {} ", tup.2);
    let array: [i64; 29]  = rand::random();
    for x in 0..array.len()
    {
        println!("{}", array[x]);
    }
}
