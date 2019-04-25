fn main() {
    fib(22);
}

fn fib(n: i64) {
    let mut a = 1;
    let mut b = 0;
    let mut temp = 0;
    for i in 0..n + 1 {
        temp = a;
        a = b;
        b = temp + b;
    }
    println!("Fibonacci number {} is {}", n, a);
}
