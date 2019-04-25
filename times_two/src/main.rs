mod liftoff;
use liftoff::liftoff;
fn main() {
    let x = 12;
    let e = 4;
    println!("{} exp {} is {}", x, e, exp(x, e));
    liftoff(19);
}
fn exp(x: i32, e: i32) -> i32{
    let mut result = x;
    for _i in 1 .. e
    {
        result = result*x;
    }
   result 
}
