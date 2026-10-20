
mod operations;
use std::env;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() < 2 {
        println!("error: expected number and operators");
        return;
    }

    let mut number = match args[0].parse::<i32>() {
        Ok(value) => value,
        Err(_) => {
            println!("error: expected number as first argument");
            return;
        }
    };

    let mut chain: Vec<fn(i32) -> i32> = Vec::new();

    for op in args.iter().skip(1) {
        match operations::build(op) {
            Ok(function) => chain.push(function),
            Err(error) => {
                println!("{}", error);
                return;
            }
        }
    }

    for operation in chain {
        number = operation(number);
    }

    println!("result: {}", number);
}
