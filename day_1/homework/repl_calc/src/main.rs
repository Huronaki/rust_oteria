use std::io::{self, Write};
mod calculator;

fn main() {
    println!("REPL calculator: type <a> <op> <b>, exit to quit");
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        if input.trim() == "exit" {
            println!("bye!");
            break;
        }

        let parts: Vec<&str> = input.split_whitespace().collect();
        if parts.len() != 3 {
            println!("error: expected <a> <op> <b>");
            continue;
        }

        let num_1: i32 = match parts[0].parse::<i32>() {
            Ok(value) => value,
            Err(_) => {
                println!("error: expected <a> as i32 number");
                continue;
            }
        };

        let num_2: i32 = match parts[2].parse::<i32>() {
            Ok(value) => value,
            Err(_) => {
                println!("error: expected <b> as i32 number");
                continue;
            }
        };

        let operators = ["+", "-", "*", "/"];
        let op = parts[1];

        if !operators.contains(&op) {
            println!("error: unknown operator {}", op);
            continue;
        }

        let result = calculator::calculation(num_1, op, num_2);
        match result {
            Some(value) => {
                println!("{} {} {} = {}", num_1, op, num_2, value);
            }
            None => {
                if op == "/" {
                    println!("error: cannot divide by zero");
                    continue;
                } else {
                    println!("error: too big values");
                    continue;
                }
            }
        }
    }
}
