fn find_nemo(sentence: &str) -> String {
    sentence
        .split_whitespace()
        .position(|word| word == "Nemo")
        .map_or("I can't find Nemo :(".to_string(), |index| format!("I found Nemo at {}!", index + 1))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_nemo() {
        assert_eq!(find_nemo("I am finding Nemo !"), "I found Nemo at 4!");
        assert_eq!(find_nemo("Nemo is me"), "I found Nemo at 1!");
        assert_eq!(find_nemo("I Nemo am"), "I found Nemo at 2!");
        assert_eq!(find_nemo("Hello world"), "I can't find Nemo :(");
        assert_eq!(find_nemo("This is Nemo's spot"), "I can't find Nemo :(");
        assert_eq!(find_nemo("Nemo Nemo Nemo"), "I found Nemo at 1!");
    }
}

fn main() {
    let sentences = vec![
        "I am finding Nemo !",
        "Nemo is me",
        "I Nemo am",
        "Hello world",
        "This is Nemo's spot",
        "Nemo Nemo Nemo"
    ];

    for sentence in sentences {
        println!("{}", find_nemo(sentence));
    }
}
