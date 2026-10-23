mod config;

use std::env;

fn main() {
    let Some(path) = env::args().nth(1) else {
        eprintln!("Usage: cargo run -- <config-file>");
        return;
    };

    match config::load(&path) {
        Ok(config) => {
            println!("target: {}", config.target);
            println!("ports: {}-{}", config.ports.0, config.ports.1);
            println!("threads: {}", config.threads);
        }
        Err(error) => {
            eprintln!("error: {error}");
        }
    }
}
