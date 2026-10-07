use std::env;
mod caesar;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        println!("Wrong count of arguments");
        return;
    }

    let text = &args[2];
    if text.is_empty() {
        println!("Text cannot be empty");
        return;
    }

    let shift: i64 = match args[3].parse::<i64>() {
        Ok(value) => value,
        Err(_) => {
            println!("Shift must be integer");
            return;
        }
    };

    let operation = &args[1];

    match operation.as_str() {
        "encrypt" => {
            let result = caesar::encrypt(text, shift);
            println!("{}", result);
        }
        "decrypt" => {
            let result = caesar::decrypt(text, shift);
            println!("{}", result);
        }
        _ => {
            println!("Wrong operation");
        }
    }
}
