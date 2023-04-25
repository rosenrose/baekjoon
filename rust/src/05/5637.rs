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
        .find_map(|w| (w.len() == longest_len).then(|| w.to_lowercase()))
        .unwrap();

    println!("{first_longest}");
}
