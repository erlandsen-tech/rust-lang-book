use rand::Rng;

fn main() {
    let penny = Coin::Penny;
    let nickel = Coin::Nickel;
    let dime = Coin::Dime;
    let quarter = Coin::Quarter;
    println!("Penny {}, Nickle {}, Dime {}, Quarter {}",
             value_in_cents(penny), value_in_cents(nickel),
             value_in_cents(dime), value_in_cents(quarter));
    let some_u8_value = Some(0u8);
    if let Some(3) = some_u8_value {
        println!("three!");
    }
}


pub enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter
}

fn value_in_cents (coin: Coin) -> u32 {
    match coin {
        Coin::Penny => {
            hooray_if_lucky();
            1},
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25
    }
}

fn hooray_if_lucky() {
    let random_number = rand::thread_rng().gen_range(1, 101) ;
    match random_number { 
        13 => println!("Lucky number thirteen!"),
        7 => println!("Number Slevin!"),
        42 => println!("The meaning of life!"),
        _ => println!("No dice.."),
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
