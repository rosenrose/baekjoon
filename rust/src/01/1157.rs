use core::iter::Iterator;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut counts = [0; 26];
    let offset = b'A';

    for ch in buf.trim().to_uppercase().as_bytes() {
        counts[(ch - offset) as usize] += 1;
    }

    let max_count = counts.iter().max().unwrap();
    let mut most_letter = '\0';

    for ch in counts
        .iter()
        .enumerate()
        .filter_map(|(ch, count)| (count == max_count).then_some((ch as u8 + offset) as char))
    {
        if most_letter != '\0' {
            println!("?");
            return;
        }

        most_letter = ch;
    }

    println!("{}", most_letter);
}
