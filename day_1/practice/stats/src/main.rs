fn analyze(grades: [f64; 4]) -> (f64, f64) {
    let mut sum: f64 = 0.0;
    let mut max: f64 = 0.0;

    for grade in grades {
        sum += grade;
        if grade > max {
            max = grade;
        }
    }
    (sum / grades.len() as f64, max)
}
fn main() {
    let grades = [12.0, 8.5, 17.0, 14.5];
    let (avg, max) = analyze(grades);

    println!("avg = {avg}, max = {max}");
}
