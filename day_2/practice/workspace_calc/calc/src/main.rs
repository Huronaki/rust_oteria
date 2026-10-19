use std::env;
use math::calculation;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        println!("error: wrong count of arguments");
        return;
    }

    let num_1: i32 = match args[1].parse::<i32>() {
        Ok(value) => value,
        Err(_) => {
            println!("error: first argument should be a 32bit numberr");
            return;
        }
    };

    let num_2: i32 = match args[3].parse::<i32>() {
        Ok(value) => value,
        Err(_) => {
            println!("error: third argument should be a 32bit number");
            return;
        }
    };

    let operators = ["+", "-", "*", "/"];
    let op = &args[2];

    if !operators.contains(&op.as_str()) {
        println!("error: unknown operator {}", op);
        return;
    }

    let result = calculation(num_1, op, num_2);

    match result {
        Some(value) => {
            println!("{} {} {} = {}", num_1, op, num_2, value);
        }
        None => {
            if op == "/" {
                println!("error: cannot divide by zero");
            } else {
                println!("error: too big values");
            }
        }
    }
}
