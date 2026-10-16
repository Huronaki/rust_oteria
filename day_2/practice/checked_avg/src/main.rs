use std::env;

fn average(nums: &[f64]) -> Result<f64, String> {
    if nums.is_empty() {
        return Err("error: no numbers given".to_string());
    }

    let mut sum = 0.0;
    for num in nums {
            sum += *num;
    }

    let avg = sum/nums.len() as f64;
    return Ok(avg);
}


fn main() {
    let argv: Vec<String> = env::args().skip(1).collect();
    let mut nums: Vec<f64> = Vec::new();

    for arg in argv {
        match arg.parse::<f64>() {
            Ok(num) => nums.push(num),
            Err(_) => {
                eprintln!("error: invalid number: '{}'", arg);
                return;
            }
        }
    }

    match average(&nums) {
        Ok(avg) => println!("{avg:.2}"),
        Err(error) => eprintln!("{error}"),
    }
}