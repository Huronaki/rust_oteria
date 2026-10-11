mod ipv4;

use ipv4::{Outcome, parse_ipv4};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("error: expected one IPv4 address");
        return;
    }

    match parse_ipv4(&args[1]) {
        Outcome::Ok(ip) => ip.show(),
        Outcome::Err(error) => error.describe(),
    }
}
