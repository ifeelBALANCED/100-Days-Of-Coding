const AVAILABLE_STEPS: [i32; 5] = [1, 2, 3, 4, 5];

fn elephant(mut n: i32) -> i32 {
    AVAILABLE_STEPS.iter().rev().fold(0, |steps, &step| {
        let step_count = n / step;
        n %= step;
        steps + step_count
    })
}

fn main() {
    let n = 12;
    let result = elephant(n);
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_elephant() {
        // Test case 1: Exact fit with largest step
        assert_eq!(elephant(5), 1);
        assert_eq!(elephant(10), 2);

        // Test case 2: Mixed steps
        assert_eq!(elephant(12), 3); // 5 + 5 + 2
        assert_eq!(elephant(23), 5); // 5 + 5 + 5 + 5 + 3

        // Test case 3: Edge cases
        assert_eq!(elephant(1), 1); // Smallest possible value
        assert_eq!(elephant(1000000), 200000); // Largest possible value

        // Test case 4: Random values
        assert_eq!(elephant(7), 2); // 5 + 2
        assert_eq!(elephant(14), 3); // 5 + 5 + 4
        assert_eq!(elephant(29), 6); // 5 + 5 + 5 + 5 + 5 + 4
        assert_eq!(elephant(999999), 200000); // 5 * 199999 + 4
    }
}