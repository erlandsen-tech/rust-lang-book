fn main() {
    println!("Hello, world!");
    function(987);
}

fn function(x: i32) {
    println!("The value of x from function() is {}", x);
    let x = five();
    println!("The value after calling five() is {}", x);
}

fn five() -> i32
{
    5
}
