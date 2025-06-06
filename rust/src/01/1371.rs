use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let input = buf.replace(char::is_whitespace, "");

    let offset = b'a';
    let mut counts = [0; 26];

    for ch in input.as_bytes() {
        counts[(ch - offset) as usize] += 1;
    }

    let max_count = counts.iter().max().unwrap();
    let max_letters = String::from_iter(
        counts
            .iter()
            .enumerate()
            .filter_map(|(ch, count)| (count == max_count).then_some((ch as u8 + offset) as char)),
    );

    println!("{max_letters}");
}
