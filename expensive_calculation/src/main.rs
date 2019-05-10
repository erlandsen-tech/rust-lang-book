use std::thread;
use std::time::Duration;
fn main() {
println!("{}", expensive_calculation(5,2));
}
pub fn expensive_calculation(to_power: i32, power: u32) -> i32 {
    println!("Calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    i32::pow(to_power, power)
}
