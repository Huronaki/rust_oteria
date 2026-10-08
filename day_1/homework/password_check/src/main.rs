mod check_password;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Wrong count of arguments");
        return;
    }

    let password = &args[1];
    if password.is_empty() {
        println!("Password cannot be empty");
        return;
    }

    let score = check_password::check_password(password);

    if let Some(score) = score {
        println!("score {}/5: {}", score.0, score.1);
    }
}
