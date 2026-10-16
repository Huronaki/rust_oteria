fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() >= b.len() { a } else { b }
}

struct Report<'a> {
    winner: &'a str,
}

use std::env;

fn main() {
    let argv: Vec<String> = env::args().skip(1).collect();

    if argv.len() != 2 {
        println!("error: expected 2 arguments <a> <b>");
        return;
    }

    let report = Report {
        winner: longest(&argv[0], &argv[1]),
    };

    println!("longest: {} ({} chars)", report.winner, report.winner.len());
}
