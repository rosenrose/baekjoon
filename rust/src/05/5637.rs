use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut longest_len = 0;

    let words: Vec<_> = buf
        .split(|c| !matches!(c, 'a'..='z' | 'A'..='Z' | '-'))
        .filter(|s| {
            longest_len = s.len().max(longest_len);
            !s.is_empty()
        })
        .collect();

    let first_longest = words
        .iter()
        .find(|w| w.len() == longest_len)
        .unwrap()
        .to_lowercase();

    println!("{first_longest}");
}
