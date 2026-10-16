use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    let numbers: Vec<i32> = args
        .iter()
        .map(|arg| {
            arg.parse::<i32>()
                .unwrap_or_else(|_| {
                    eprintln!("error: invalid number '{}'", arg);
                    process::exit(1);
                })
        })
        .collect();

    let sum: i32 = numbers.iter().sum();

    let evens: Vec<i32> = numbers
        .iter()
        .filter(|&&n| n % 2 == 0)
        .copied()
        .collect();

    let squares: Vec<i32> = numbers
        .iter()
        .map(|&n| n * n)
        .collect();

    println!("sum: {}", sum);
    println!(
        "evens: {}",
        evens.iter().map(|n| n.to_string()).collect::<Vec<_>>().join(" ")
    );
    println!(
        "squares: {}",
        squares.iter().map(|n| n.to_string()).collect::<Vec<_>>().join(" ")
    );
}
