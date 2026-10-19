use std::collections::HashMap;
use std::fs;

pub fn scan(file_name: &str) -> Result<HashMap<String, u32>, String> {
    let text = fs::read_to_string(file_name)
        .map_err(|_| format!("error: cannot read file '{}'", file_name))?;
    let mut res: HashMap<String, u32> = HashMap::new();

    for line in text.lines() {
        let tokens: Vec<&str> = line.split_whitespace().collect();

        let level = tokens.get(2).unwrap_or(&"other");
        let key = match *level {
            "ERROR" => "ERROR",
            "INFO" => "INFO",
            "WARN" => "WARN",
            _ => "other",
        };

        *res.entry(key.to_string()).or_insert(0) += 1;
    }
    Ok(res)
}
