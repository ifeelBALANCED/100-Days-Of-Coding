use std::collections::HashMap;

fn sock_pairs(sock_string: &str) -> i32 {
    sock_string
        .chars()
        .fold(HashMap::new(), |mut sock_count_map, sock_type| {
            *sock_count_map.entry(sock_type).or_insert(0) += 1;
            sock_count_map
        })
        .values()
        .map(|&sock_count| sock_count / 2).sum::<i32>()
}

fn main() {
    let pairs = "AA";
    let result = sock_pairs(pairs);
    println!("{:?}", result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sock_pairs() {
        assert_eq!(sock_pairs("AA"), 1);
        assert_eq!(sock_pairs("ABABC"), 2);
        assert_eq!(sock_pairs("CABBACCC"), 4);
        assert_eq!(sock_pairs(""), 0);
        assert_eq!(sock_pairs("A"), 0);
        assert_eq!(sock_pairs("ABCDEF"), 0);
        assert_eq!(sock_pairs("AABBAABB"), 4);
        assert_eq!(sock_pairs(&"A".repeat(10000)), 5000);
    }
}