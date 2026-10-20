pub fn inc(n: i32) -> i32 {
    n + 1
}

pub fn double(n: i32) -> i32 {
    n * 2
}

pub fn square(n: i32) -> i32 {
    n * n
}

pub fn flip(n: i32) -> i32 {
    -n
}

pub fn build(op: &str) -> Result<fn(i32) -> i32, String> {
    match op {
        "inc" => Ok(inc),
        "double" => Ok(double),
        "square" => Ok(square),
        "flip" => Ok(flip),
        _ => Err(format!("error: unknown op '{}'", op)),
    }
}