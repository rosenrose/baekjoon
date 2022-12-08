use std::collections::HashMap;
use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut input = buf.split_ascii_whitespace();
    let n: i32 = input.next().unwrap().parse().unwrap();

    let char_count = |s: &str| -> HashMap<_, _> {
        s.chars()
            .map(|c| (c, s.matches(c).count() as i32))
            .collect()
    };

    for _ in 0..n {
        let (a, b) = (input.next().unwrap(), input.next().unwrap());

        println!(
            "{a} & {b} are {}",
            if char_count(a) == char_count(b) {
                "anagrams."
            } else {
                "NOT anagrams."
            }
        );
    }
}
