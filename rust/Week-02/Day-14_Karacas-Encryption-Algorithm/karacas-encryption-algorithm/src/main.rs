fn encrypt_karacas(to_encrypt: &str) -> String {
    let mut result = String::with_capacity(to_encrypt.len() + 3);
    for c in to_encrypt.chars().rev() {
        result.push(match c {
            'a' => '0',
            'e' => '1',
            'i' | 'o' => '2',
            'u' => '3',
            _ => c,
        });
    }
    result.push_str("aca");
    result
}

fn main() {
    let input = "apple";
    let encrypted = encrypt_karacas(input);
    println!("Encrypted: {}", encrypted);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_karacas() {
        assert_eq!(encrypt_karacas("banana"), "0n0n0baca");
        assert_eq!(encrypt_karacas("karaca"), "0c0r0kaca");
        assert_eq!(encrypt_karacas("burak"), "k0r3baca");
        assert_eq!(encrypt_karacas("alpaca"), "0c0pl0aca");
    }
}