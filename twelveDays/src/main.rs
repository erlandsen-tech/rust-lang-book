fn main() {
    let what_she_gave_me = [
        "partridge in a pear tree",
        "turtle doves",
        "french hens",
        "calling birds",
        "gold rings",
        "geese a laying",
        "swans a swimming",
        "maids a milking",
        "ladies dancing",
        "lords a leaping",
        "pipers piping",
        "drummers drumming",
    ];
    let numbers = [
        "a", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine", "Ten", "Eleven",
        "Twelve",
    ];
    let positions = [
        "first", "second", "third", "fourht", "fifth", "sixth", "seventh", "eight", "ninth",
        "tenth", "eleventh", "twelfth",
    ];
    for i in 0..12 {
        print!("On the {} day of christmas \n", positions[i]);
        println!("My true love gave to me");
        let mut j = i;
        while j >= 1 {
            println!("{} {}", numbers[j], what_she_gave_me[j]);
            j = j - 1;
        }
        if i != 0 {
            println!("and {} {}", numbers[0], what_she_gave_me[0]);
        }
        println!("");
    }
}
