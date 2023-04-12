use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();

    for input in buf.lines().skip(1) {
        let mut letters = [false; 26];

        for ch in input.to_lowercase().chars() {
            if ch.is_alphabetic() {
                letters[ch as usize - 'a' as usize] = true;
            }
        }

        if letters.iter().all(|&b| b) {
            println!("pangram");
            continue;
        }

        let missing_letters: String = letters
            .iter()
            .enumerate()
            .filter_map(|(ch, &b)| (!b).then_some((ch as u8 + 'a' as u8) as char))
            .collect();

        println!("missing {missing_letters}");
    }
}
