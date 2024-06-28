fn progress_days(days: Vec<i32>) -> i32 {
    days.windows(2)
        .filter(|window| window[0] < window[1])
        .count() as i32
}

fn main() {
    let days = vec![3, 4, 1, 2];

    let result = progress_days(days);

    println!("{:?}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_progress_days() {
        assert_eq!(progress_days(vec![3, 4, 1, 2]), 2);
        assert_eq!(progress_days(vec![10, 11, 12, 9, 10]), 3);
        assert_eq!(progress_days(vec![6, 5, 4, 3, 2, 9]), 1);
        assert_eq!(progress_days(vec![9, 9]), 0);
    }
}