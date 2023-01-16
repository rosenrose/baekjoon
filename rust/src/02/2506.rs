use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let score: usize = buf
        .lines()
        .next_back()
        .unwrap()
        .replace(' ', "")
        .split(|c| c == '0')
        .filter_map(|s| (!s.is_empty()).then_some(s.char_indices().map(|(i, _)| i + 1)))
        .flatten()
        .sum();

    println!("{score}");
}
