fn merge_sorted_array(nums1: &mut Vec<i64>, m: usize, nums2: &mut Vec<i64>, n: usize) {
    nums1.truncate(m);
    nums1.extend_from_slice(&nums2[0..n]);

    nums1.sort();
}

fn main() {
    let mut nums1 = vec![1, 2, 3, 0, 0, 0];
    let m = 3;
    let mut nums2 = vec![2, 5, 6];
    let n = 3;

    merge_sorted_array(&mut nums1, m, &mut nums2, n);

    println!("{:?}", nums1); // Output: [1, 2, 2, 3, 5, 6]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut nums1 = vec![1, 2, 3, 0, 0, 0];
        let m = 3;
        let mut nums2 = vec![2, 5, 6];
        let n = 3;

        merge_sorted_array(&mut nums1, m, &mut nums2, n);

        assert_eq!(nums1, vec![1, 2, 2, 3, 5, 6]);
    }

    #[test]
    fn test_2() {
        let mut nums1 = vec![1];
        let m = 1;
        let mut nums2 = vec![];
        let n = 0;

        merge_sorted_array(&mut nums1, m, &mut nums2, n);

        assert_eq!(nums1, vec![1]);
    }

    #[test]
    fn test_3() {
        let mut nums1 = vec![0];
        let m = 0;
        let mut nums2 = vec![1];
        let n = 1;

        merge_sorted_array(&mut nums1, m, &mut nums2, n);

        assert_eq!(nums1, vec![1]);
    }

    #[test]
    fn test_empty_nums1_and_nums2() {
        let mut nums1 = vec![];
        let m = 0;
        let mut nums2 = vec![];
        let n = 0;

        merge_sorted_array(&mut nums1, m, &mut nums2, n);

        assert_eq!(nums1, vec![]);
    }

    #[test]
    fn test_all_elements_same() {
        let mut nums1 = vec![2, 2, 2, 0, 0, 0];
        let m = 3;
        let mut nums2 = vec![2, 2, 2];
        let n = 3;

        merge_sorted_array(&mut nums1, m, &mut nums2, n);

        assert_eq!(nums1, vec![2, 2, 2, 2, 2, 2]);
    }

    #[test]
    fn test_large_numbers() {
        let mut nums1 = vec![1, 3, 5, 0, 0, 0];
        let m = 3;
        let mut nums2 = vec![2, 4, 6];
        let n = 3;

        merge_sorted_array(&mut nums1, m, &mut nums2, n);

        assert_eq!(nums1, vec![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_very_large_numbers() {
        let mut nums1 = vec![1_000_000_000, 2_000_000_000, 3_000_000_000];
        let mut nums2 = vec![4_000_000_000, 5_000_000_000, 6_000_000_000];

        merge_sorted_array(&mut nums1, 3, &mut nums2, 3);

        assert_eq!(nums1, vec![1_000_000_000, 2_000_000_000, 3_000_000_000, 4_000_000_000, 5_000_000_000, 6_000_000_000]);
    }

    #[test]
    fn test_large_negative_numbers() {
        let mut nums1 = vec![-3_000_000_000, -2_000_000_000, -1_000_000_000];
        let mut nums2 = vec![-6_000_000_000, -5_000_000_000, -4_000_000_000];

        merge_sorted_array(&mut nums1, 3, &mut nums2, 3);

        assert_eq!(nums1, vec![-6_000_000_000, -5_000_000_000, -4_000_000_000, -3_000_000_000, -2_000_000_000, -1_000_000_000]);
    }
}
