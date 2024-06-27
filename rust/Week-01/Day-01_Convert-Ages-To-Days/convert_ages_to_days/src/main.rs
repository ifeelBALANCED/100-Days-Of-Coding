use std::io::{self, Write};

fn calc_age(age: u32) -> u32 {
    age * 365
}

fn main() {
    print!("Enter your age: ");
    io::stdout().flush().unwrap(); // Ensure prompt is printed before user input

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    match input.trim().parse::<u32>() {
        Ok(age) => println!("Your age in days is: {}", calc_age(age)),
        Err(_) => eprintln!("Please enter a valid number"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calc_age() {
        assert_eq!(calc_age(65), 23725);
        assert_eq!(calc_age(0), 0);
        assert_eq!(calc_age(20), 7300);
        assert_eq!(calc_age(1), 365);
        assert_eq!(calc_age(100), 36500);
    }
}
