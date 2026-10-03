mod units;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let unit = &args[1];
    let value: f64 = args[2].parse().unwrap();

    match unit.as_str() {
        "km" => {
            let miles = units::length::km_to_miles(value);
            println!("{} km = {:.2} miles", value, miles)
        }
        "kg" => {
            let lbs = units::mass::kg_to_lbs(value);
            println!("{} kg = {:.2} lbs", value, lbs)
        }
        _ => {
            println!("unknown unit: {}", unit);
        }
    }
}
