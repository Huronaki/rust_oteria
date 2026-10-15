mod ipv4;
mod ipv6;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("error: expected one IP address");
        return;
    }

    let input = &args[1];

    if input.contains(':') {
        match ipv6::parse_ipv6(input) {
            ipv6::Outcome::Ok(ip) => ip.show(),
            ipv6::Outcome::Err(error) => error.describe(),
        }
    } else {
        match ipv4::parse_ipv4(input) {
            ipv4::Outcome::Ok(ip) => ip.show(),
            ipv4::Outcome::Err(error) => error.describe(),
        }
    }
}
