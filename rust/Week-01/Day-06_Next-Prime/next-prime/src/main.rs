fn is_prime(num: u64) -> bool {
    if num <= 1 {
        return false;
    }
    if num <= 3 {
        return true;
    }
    if num % 2 == 0 || num % 3 == 0 {
        return false;
    }

    let mut i = 5;
    while i * i <= num {
        if num % i == 0 || num % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }
    true
}

fn next_prime(mut number: u64) -> u64 {
    if number <= 1 {
        return 2;
    }
    if number == 2 {
        return 3;
    }

    if number % 2 == 0 {
        number += 1;
    } else {
        number += 2;
    }

    while !is_prime(number) {
        number += 2;
    }

    number
}

fn main() {
    let number = 99999999977;
    let next_prime_number = next_prime(number);
    println!("Next prime number after {} is {}", number, next_prime_number);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_prime() {
        assert_eq!(next_prime(12), 13);
        assert_eq!(next_prime(24), 29);
        assert_eq!(next_prime(11), 13);
        assert_eq!(next_prime(13), 17);
        assert_eq!(next_prime(19), 23);
    }

    #[test]
    fn test_next_prime_large_numbers() {
        assert_eq!(next_prime(4294967291), 4294967311);
        assert_eq!(next_prime(4294967296), 4294967311);
        assert_eq!(next_prime(99999999977), 100000000003);
    }
}
