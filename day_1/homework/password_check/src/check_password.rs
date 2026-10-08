// 8 char -> 1 point
// 12 char -> 1 point
// lower+upper case -> 1 point
// special character (not number/letter) -> 1 poin
// number -> 1 point
//--------------------
// 0-2 -> weak
// 3-4 -> medium
// 5 -> strong
//--------------------
// read agrument
// ouptut:
//   score x/5: weack/medium/strong

pub fn check_password(password: &str) -> Option<(u8, &'static str)> {
    let results: [(u8, &'static str); 6] = [
        (0, "weak"),
        (1, "weak"),
        (2, "weak"),
        (3, "medium"),
        (4, "medium"),
        (5, "strong"),
    ];

    let mut num: u64 = 0;
    let mut lower: u64 = 0;
    let mut upper: u64 = 0;
    let mut special: u64 = 0;

    for c in password.chars() {
        if c.is_ascii_digit() {
            num += 1;
        } else if c.is_ascii_lowercase() {
            lower += 1;
        } else if c.is_ascii_uppercase() {
            upper += 1;
        } else if !c.is_ascii() {
            println!("Enter only ascii symbols");
            return None;
        } else {
            special += 1;
        }
    }

    let mut strength_score: u8 = 0;
    if password.len() >= 8 {
        strength_score += 1;
    }
    if password.len() >= 12 {
        strength_score += 1;
    }
    if num > 0 {
        strength_score += 1;
    }
    if lower > 0 && upper > 0 {
        strength_score += 1;
    }
    if special > 0 {
        strength_score += 1;
    }

    let result = results[strength_score as usize];
    Some(result)
}
