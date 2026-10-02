fn longer_len(a: &str, b: &str) -> usize {
    if a.len() > b.len() { a.len() } else { b.len() }
}

fn main() {
    let lang = String::from("rust systems");

    let result = longer_len(&lang, "hello wordl");
    println!("longer: {result} chars");

    let first = &lang[..4];
    let second = &lang[5..];
    println!("{first} / {second}");

    println!("still owned: {lang}");
}
