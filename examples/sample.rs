pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn multiply(x: f64, y: f64) -> f64 {
    x * y
}

pub fn is_even(n: u32) -> bool {
    n % 2 == 0
}

fn main() {
    println!("Sample functions for testing");
    println!("add(2, 3) = {}", add(2, 3));
    println!("multiply(2.5, 4.0) = {}", multiply(2.5, 4.0));
    println!("is_even(4) = {}", is_even(4));
}