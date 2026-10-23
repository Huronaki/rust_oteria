mod hexdump;

use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: cargo run -- <file>");
        return;
    }

    let bytes = match fs::read(&args[1]) {
        Ok(bytes) => bytes,
        Err(error) => {
            eprintln!("Error reading file: {error}");
            return;
        }
    };

    print!("{}", hexdump::format(&bytes));
}
