mod display;
use display::longest_with_an_announcement as to_long;
fn main() {
    let string1 = String::from("long string is long");
    //By using brackets we define a lifetime for variables
    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }

    let novel = String::from("Call med Ishmael. bla bla bla ... ");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportanExcerpt {
        part: first_sentence,
    };
    println!("part: {}", i.part);
    println!("{}", to_long(&novel, i.part, "ANN"));
}
// 'a is lifetime specifier. must be specified so that we
// know when to drop the data
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

struct ImportanExcerpt<'a> {
    part: &'a str,
}
