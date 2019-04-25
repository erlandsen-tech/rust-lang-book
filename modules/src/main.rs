fn main() {
    sound::keys::acoustic::piano();

    let mut v = plant::Vegetable::new("Agurk");

    v.name = String::from("Paprika");
    println!("{} er sinnssykt gode", v.name);
    sound::organic::voice();
}
mod sound {
    pub mod keys {
        pub mod electric {
            pub fn keyboard() {
                println!("pling plong");
            }
        }
        pub mod acoustic {
            pub fn piano() {
                println!("ding dong");
                super::push_key();
            }
        }
        fn push_key() {
            println!("tchk");
        }
    }
    pub mod organic {
        pub fn voice() {
            println!("oo ah");
        }
    }
}
mod plant {
    pub struct Vegetable {
        pub name: String,
        id: i32,
    }

    impl Vegetable {
        pub fn new(name: &str) -> Vegetable {
            Vegetable {
                name: String::from(name),
                id: 1,
            }
        }
    }
}
