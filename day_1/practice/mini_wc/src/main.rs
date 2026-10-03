use std::collections::HashMap;

fn main() {
    let words = ["rust", "is", "fast", "rust", "is", "safe"];
    let mut counts: HashMap<&str, i32> = HashMap::new();

    for w in words {
        let count = counts.entry(w).or_insert(0);
        *count += 1;
    }

    let mut keys: Vec<_> = counts.keys().collect();
    keys.sort();

    for word in keys {
        let count = counts.get(word).unwrap();
        println!("{}: {}", word, count);
    }
}
