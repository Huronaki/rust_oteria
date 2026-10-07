pub fn encrypt(msg: &str, shift: i64) -> String {
    let mut result = String::new();

    for symbol in msg.chars() {
        if symbol.is_ascii_lowercase() {
            let number = symbol as i64 - 'a' as i64;
            let shifted = (number + shift).rem_euclid(26);
            let new_symbol = (shifted + 'a' as i64) as u8 as char;

            result.push(new_symbol);
        } else if symbol.is_ascii_uppercase() {
            let number = symbol as i64 - 'A' as i64;
            let shifted = (number + shift).rem_euclid(26);
            let new_symbol = (shifted + 'a' as i64) as u8 as char;

            result.push(new_symbol);
        } else {
            result.push(symbol);
        }
    }

    result
}

pub fn decrypt(msg: &str, shift: i64) -> String {
    let mut result = String::new();

    for symbol in msg.chars() {
        if symbol.is_ascii_lowercase() {
            let number = symbol as i64 - 'a' as i64;
            let shifted = (number - shift).rem_euclid(26);
            let new_symbol = (shifted + 'a' as i64) as u8 as char;

            result.push(new_symbol);
        } else if symbol.is_ascii_uppercase() {
            let number = symbol as i64 - 'A' as i64;
            let shifted = (number - shift).rem_euclid(26);
            let new_symbol = (shifted + 'a' as i64) as u8 as char;

            result.push(new_symbol);
        } else {
            result.push(symbol);
        }
    }

    result
}
