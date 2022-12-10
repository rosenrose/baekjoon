use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let score: usize = buf
        .split_ascii_whitespace()
        .skip(1)
        .collect::<String>()
        .split(|c| c == '0')
        .filter_map(|s| (!s.is_empty()).then(|| s.char_indices().map(|(i, _)| i + 1)))
        .flatten()
        .sum();

    println!("{score}");
}
