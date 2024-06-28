fn barbecue_skewers(skewers: Vec<&str>) -> Vec<usize> {
    let all_skewers = skewers.len();
    let vegetarian = skewers.iter().filter(|&skewer| !skewer.contains('x')).count();

    vec![
        vegetarian,
        all_skewers - vegetarian
    ]
}

fn main() {
    let test_skewers = vec![
        "--xo--x--ox--",
        "--xx--x--xx--",
        "--oo--o--oo--",
        "--xx--x--ox--",
        "--xx--x--ox--"
    ];

    let result = barbecue_skewers(test_skewers);
    println!("{:?}", result); // Output should be: [1, 4]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_barbecue_skewers() {
        assert_eq!(
            barbecue_skewers(vec![
                "--xo--x--ox--",
                "--xx--x--xx--",
                "--oo--o--oo--",
                "--xx--x--ox--",
                "--xx--x--ox--"
            ]),
            vec![1, 4]
        );

        assert_eq!(
            barbecue_skewers(vec![
                "--oooo-ooo--",
                "--xx--x--xx--",
                "--o---o--oo--",
                "--xx--x--ox--",
                "--xx--x--ox--"
            ]),
            vec![2, 3]
        );

        assert_eq!(
            barbecue_skewers(vec![
                "--oooo-ooo--",
                "--xxxxxxxx--",
                "--o---",
                "-o-----o---x--",
                "--o---o-----"
            ]),
            vec![3, 2]
        );

        // Additional test cases
        assert_eq!(
            barbecue_skewers(vec![
                "----------",
                "----------",
                "----------"
            ]),
            vec![3, 0]
        );

        assert_eq!(
            barbecue_skewers(vec![
                "--x--x--x--",
                "--x--x--x--",
                "--x--x--x--"
            ]),
            vec![0, 3]
        );

        assert_eq!(
            barbecue_skewers(vec![
                "--x--x--x--",
                "-----------",
                "--x--x--x--",
                "-----------"
            ]),
            vec![2, 2]
        );

        assert_eq!(
            barbecue_skewers(vec![
                "x",
                "o",
                "x-o-x"
            ]),
            vec![1, 2]
        );

        assert_eq!(
            barbecue_skewers(vec![
                "oxoxoxoxo",
                "xoxoxoxox",
                "ooxxoo",
                "xxoo",
                "oooxxxooo"
            ]),
            vec![0, 5]
        );
    }
}
