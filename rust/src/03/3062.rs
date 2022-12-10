use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let input = buf.lines().map(|s| s.parse::<i32>().unwrap());

    for num in input.skip(1) {
        let sum = num
            + num
                .to_string()
                .chars()
                .rev()
                .collect::<String>()
                .parse::<i32>()
                .unwrap();

        println!(
            "{}",
            if is_palindrome(&sum.to_string()[..]) {
                "YES"
            } else {
                "NO"
            }
        );
    }
}

fn is_palindrome(word: &str) -> bool {
    let len = word.len();

    if len <= 1 {
        return true;
    }

    if word.chars().nth(0) != word.chars().nth(len - 1) {
        return false;
    }

    is_palindrome(&word[1..len - 1])
}
