fn shout(name: String) -> String {
    name.to_uppercase()
}

fn double(n: i32) -> i32 {
    n * 2
}

fn main() {
    let name = String::from("alice");
    println!("before: {name}");

    let loud = shout(name);
    println!("after: {loud}");

    let n = 7;
    let result = double(n);

    println!("{n} doubled = {result}");
}
