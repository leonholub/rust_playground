//! Fibonacci
//!
//! compile and run with `rustc fibonacci.rs && ./fibonacci`
//!

use std::io;

/// main function of console application
fn main(){
    println!("Fibonacci generator\nHow many numbers do you want? ");

    let mut input = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut input).expect("error: unable to read user input");

    let trimmed = input.trim();
    match trimmed.parse::<u32>(){
        Ok(n) => {
            println!("There you go: {} numbers..", n);

            for i in 0..n{
                print!("{} ", fibonacci(i));
            }

            println!("");
        },
        Err(..) => println!("You entered '{}' and that's not a number..", trimmed)
    }
}

/// recursive fibonacci solver, returns the n-th fibonacci number
fn fibonacci(n: u32) -> u64{
    match n {
        1 | 2 => 1,
        0 => 0,
        _ => fibonacci(n-1) + fibonacci(n-2)
    }
}