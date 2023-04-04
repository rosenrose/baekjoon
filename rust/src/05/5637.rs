use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut longest_len = 0;

    let words: Vec<_> = buf
        .split(|ch| !matches!(ch, 'a'..='z' | 'A'..='Z' | '-'))
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
