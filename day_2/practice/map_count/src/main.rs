use std::collections::HashMap;
use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() != 1 {
        eprintln!("error: expected one argument: <file>");
        process::exit(1);
    }

    let path = &args[0];

    let contents = fs::read_to_string(path).unwrap_or_else(|_| {
        eprintln!("error: cannot read file '{}'", path);
        process::exit(1);
    });

    let mut word_counts: HashMap<String, u32> = HashMap::new();

    for word in contents
        .split(|c: char| !c.is_alphanumeric())
        .filter(|word| !word.is_empty())
    {
        let word = word.to_lowercase();
        *word_counts.entry(word).or_insert(0) += 1;
    }

    let mut words: Vec<_> = word_counts.iter().collect();
    words.sort_by_key(|(word, _)| *word);

    for (word, count) in words {
        println!("{}: {}", word, count);
    }
}
