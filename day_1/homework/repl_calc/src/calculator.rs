pub fn calculation(a: i32, op: &str, b: i32) -> Option<i32> {
    match op {
        "+" => a.checked_add(b),
        "-" => a.checked_sub(b),
        "*" => a.checked_mul(b),
        "/" => a.checked_div(b),
        _ => None,
    }
}
