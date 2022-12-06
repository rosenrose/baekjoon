use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let input = buf.split_ascii_whitespace();

    for num in input.take_while(|&num| num != "0") {
        println!("{}", if is_palindrome(num) { "yes" } else { "no" });
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
