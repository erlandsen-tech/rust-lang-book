mod sphere;
use sphere::*;
fn main() {
    let ball = Sphere{ radius: 3.2, height: 12.1};
    let ball2 = Sphere{ radius: 4.2, height: 8.1};
    let ball3 = Sphere{ radius: 1.2, height: 2.1};
    println!("attrbutes of ball are: {:?}", &ball);
    println!("are of ball is: {}", ball.area());
    println!("ball can hold ball3: {}", ball.can_hold(&ball3));
    println!("ball3 can hold ball2: {}", ball3.can_hold(&ball2));
    println!("ball2 can hold ball: {}", ball2.can_hold(&ball));
}
