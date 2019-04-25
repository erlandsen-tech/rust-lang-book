pub fn takes_and_gives(mut string: String) -> String
{
    println!("{}", string);
    string.push_str("  have it back mate");
    string
}

pub fn change(string: &mut String) {
    string.push_str(", world");
}
