fn main() {
    let mut counter = -10;

    let mut result = loop {
        counter += 3;
        if counter > 92 {
            break counter * 5;
        }
    };
    println!("{}", counter);
    println!("{}", result);

    while result != 0
    {
        println!("result = {}", result);
        result = result - 1;
    }
    for_loop(100);
}

fn for_loop(i: u32) {
    for x in 0..i {
        println!("{}", x);
    }
}
