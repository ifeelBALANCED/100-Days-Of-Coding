fn letter_combinations_of_a_phone_number(digits: String) -> Vec<String> {
    if digits.is_empty() {
        return vec![];
    }

    let digits_to_char = [
        "", // 0
        "", // 1
        "abc", // 2
        "def", // 3
        "ghi", // 4
        "jkl", // 5
        "mno", // 6
        "pqrs", // 7
        "tuv", // 8
        "wxyz", // 9
    ];

    let mut letter_combinations = vec!["".to_string()];

    for digit in digits.chars() {
        if let Some(&chars) = digit.to_digit(10).and_then(|d| digits_to_char.get(d as usize)) {
            letter_combinations = letter_combinations
                .iter()
                .flat_map(|prefix| chars.chars().map(move |ch| format!("{}{}", prefix, ch)))
                .collect()
        }
    }

    letter_combinations
}

fn main() {
    let digits = "23".to_string();
    let result = letter_combinations_of_a_phone_number(digits);
    println!("{:?}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_string() {
        let digits = "".to_string();
        let result = letter_combinations_of_a_phone_number(digits);
        assert_eq!(result, Vec::<String>::new());
    }

    #[test]
    fn test_single_digit() {
        let digits = "2".to_string();
        let result = letter_combinations_of_a_phone_number(digits);
        let expected = vec!["a".to_string(), "b".to_string(), "c".to_string()];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_multiple_digits() {
        let digits = "23".to_string();
        let result = letter_combinations_of_a_phone_number(digits);
        let expected = vec![
            "ad".to_string(), "ae".to_string(), "af".to_string(),
            "bd".to_string(), "be".to_string(), "bf".to_string(),
            "cd".to_string(), "ce".to_string(), "cf".to_string(),
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_digits_with_four_letters() {
        let digits = "79".to_string();
        let result = letter_combinations_of_a_phone_number(digits);
        let expected = vec![
            "pw".to_string(), "px".to_string(), "py".to_string(), "pz".to_string(),
            "qw".to_string(), "qx".to_string(), "qy".to_string(), "qz".to_string(),
            "rw".to_string(), "rx".to_string(), "ry".to_string(), "rz".to_string(),
            "sw".to_string(), "sx".to_string(), "sy".to_string(), "sz".to_string(),
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_combination_of_digits() {
        let digits = "273".to_string();
        let result = letter_combinations_of_a_phone_number(digits);
        let expected = vec![
            "apd".to_string(), "ape".to_string(), "apf".to_string(),
            "aqd".to_string(), "aqe".to_string(), "aqf".to_string(),
            "ard".to_string(), "are".to_string(), "arf".to_string(),
            "asd".to_string(), "ase".to_string(), "asf".to_string(),
            "bpd".to_string(), "bpe".to_string(), "bpf".to_string(),
            "bqd".to_string(), "bqe".to_string(), "bqf".to_string(),
            "brd".to_string(), "bre".to_string(), "brf".to_string(),
            "bsd".to_string(), "bse".to_string(), "bsf".to_string(),
            "cpd".to_string(), "cpe".to_string(), "cpf".to_string(),
            "cqd".to_string(), "cqe".to_string(), "cqf".to_string(),
            "crd".to_string(), "cre".to_string(), "crf".to_string(),
            "csd".to_string(), "cse".to_string(), "csf".to_string(),
        ];
        assert_eq!(result, expected);
    }
}