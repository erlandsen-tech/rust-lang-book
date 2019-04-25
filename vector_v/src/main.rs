use rand::Rng;

fn main() {
    // _ sets this to init var
    let v: Vec<i32> = Vec::new();
    let mut v = Vec::new();
    for i in 0..100 {
        v.push(rand::thread_rng().gen_range(0, 999));
    }
    for i in 0..101 {
        get_number(&v, i);
    }
}

fn short_scope_vec() {
    let v = vec![1, 2, 3, 4];
    for i in 0..v.len() {
        println!("{}", v[i]);
    }
}

fn get_number(vector: &Vec<i32>, number: usize) {
    match vector.get(number) {
        Some(number) => println!("The element is {}", number),
        None => println!("There is no {} number", number),
    }
}

