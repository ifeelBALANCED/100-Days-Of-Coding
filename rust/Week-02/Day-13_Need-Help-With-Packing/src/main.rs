const MAX_CARRY: i32 = 10;

fn can_fit(packages: Vec<i32>, num_bags: i32) -> bool {
    let total_weight: i32 = packages.iter().sum();
    total_weight <= num_bags * MAX_CARRY
}

fn main() {
    let packages = vec![2, 7, 1, 3, 3, 4, 7, 4, 1, 8, 2];
    let num_bags = 4;
    println!("{}", can_fit(packages, num_bags));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_fit() {
        // Test case 1: Exact fit
        let packages = vec![2, 1, 2, 5, 4, 3, 6, 1, 1, 9, 3, 2];
        let num_bags = 4;
        assert_eq!(can_fit(packages.clone(), num_bags), true);

        // Test case 2: Not enough bags
        let packages = vec![2, 7, 1, 3, 3, 4, 7, 4, 1, 8, 2];
        let num_bags = 4;
        assert_eq!(can_fit(packages.clone(), num_bags), false);

        // Test case 3: Just enough bags
        let packages = vec![10, 10, 10, 10];
        let num_bags = 4;
        assert_eq!(can_fit(packages.clone(), num_bags), true);

        // Test case 4: More bags than needed
        let packages = vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1];
        let num_bags = 10;
        assert_eq!(can_fit(packages.clone(), num_bags), true);

        // Test case 5: Edge case with maximum capacity
        let packages = vec![10; 100];
        let num_bags = 10;
        assert_eq!(can_fit(packages.clone(), num_bags), false);

        // Test case 6: Edge case with zero packages
        let packages = vec![];
        let num_bags = 1;
        assert_eq!(can_fit(packages.clone(), num_bags), true);
    }
}