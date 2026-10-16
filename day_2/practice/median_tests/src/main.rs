fn mean(nums: &[i32]) -> Option<f64> {
    if nums.is_empty() {
        return None;
    }

    let sum: i64 = nums.iter().map(|&n| n as i64).sum();
    Some(sum as f64 / nums.len() as f64)
}

fn median(nums: &[i32]) -> Option<f64> {
    if nums.is_empty() {
        return None;
    }

    let mut sorted = nums.to_vec();
    sorted.sort_unstable();

    let mid = sorted.len() / 2;

    if sorted.len() % 2 == 1 {
        Some(sorted[mid] as f64)
    } else {
        Some((sorted[mid - 1] as f64 + sorted[mid] as f64) / 2.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mean_of_three() {
        assert_eq!(mean(&[2, 4, 6]), Some(4.0));
    }

    #[test]
    fn mean_of_empty_is_none() {
        assert_eq!(mean(&[]), None);
    }

    #[test]
    fn median_odd_list() {
        assert_eq!(median(&[9, 1, 5]), Some(5.0));
    }

    #[test]
    fn median_even_list() {
        assert_eq!(median(&[7, 1, 5, 3]), Some(4.0));
    }

    #[test]
    fn median_of_empty_is_none() {
        assert_eq!(median(&[]), None);
    }
}

fn main() {
    let data = [4, 1, 3, 7, 9];

    println!("data: {:?}", data);
    println!("mean: {:.2}", mean(&data).unwrap());
    println!("median: {:.2}", median(&data).unwrap());
}