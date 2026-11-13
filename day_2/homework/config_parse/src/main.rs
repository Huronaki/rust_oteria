mod config;
use std::env;
use std::fs;

fn main() {
    let mut args: Vec<String> = env::args().skip(1).collect();

    if args.len() != 1 {
        println!("error: expected only <file name>.log");
        return;
    }

    let path = args.remove(0);

    let content = match fs::read_to_string(&path) {
        Ok(content) => content,
        Err(error) => {
            eprintln!("Cannot read config: {error}");
            return;
        }
    };

    match config::parse(&content) {
        Ok(config) => {
            println!("Target: {}", config.target);
            println!("Ports: {}-{}", config.ports.0, config.ports.1);
            println!("Threads: {}", config.threads);
        }
        Err(errors) => {
            for error in errors {
                match error {
                    config::ConfigError::Line(line, message) => {
                        eprintln!("Line {}: {}", line, message);
                    }
                    config::ConfigError::Missing(key) => {
                        eprintln!("Missing key: {}", key);
                    }
                }
            }
        }
    }
}
