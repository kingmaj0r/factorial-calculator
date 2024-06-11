use std::io;
use num::bigint::BigInt;
use num::One;

fn main() {
    println!("Enter a number to calculate its factorial:");

    let mut input = String::new();

    io::stdin().read_line(&mut input)
        .expect("Failed to read line");

    let num: u64 = match input.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Please enter a valid number!");
            return;
        }
    };

    let factorial = calculate_factorial(num);

    println!("Factorial of {} is: {}", num, factorial);
}

fn calculate_factorial(n: u64) -> BigInt {
    let mut result = BigInt::one();

    for i in 2..=n {
        result *= i;
    }

    result
}
