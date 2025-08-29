fn calculate_sum(a: i32, b: i32) -> i32 {
    a + b
}

fn validate_email(email: &str) -> bool {
    email.contains('@')
}

fn main() {
    println!("{}", calculate_sum(2, 3));
}