use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let offset = b'a';

    for input in buf.lines().skip(1) {
        let mut chars = [false; 26];

        for ch in input.to_lowercase().chars() {
            if ch.is_alphabetic() {
                chars[(ch as u8 - offset) as usize] = true;
            }
        }

        if chars.iter().all(|&b| b) {
            println!("pangram");
            continue;
        }

        let missing_chars: String = chars
            .iter()
            .enumerate()
            .filter_map(|(ch, &b)| (!b).then_some((ch as u8 + offset) as char))
            .collect();

        println!("missing {missing_chars}");
    }
}
