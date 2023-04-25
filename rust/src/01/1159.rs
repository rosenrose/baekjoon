use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut char_count = [0; 26];

    for ch in buf.lines().skip(1).flat_map(|s| s.chars().nth(0)) {
        char_count[ch as usize - 'a' as usize] += 1;
    }

    let mut availables: Vec<_> = char_count
        .iter()
        .enumerate()
        .filter_map(|(ch, &count)| (count >= 5).then_some(ch as u8 + 'a' as u8))
        .collect();

    if availables.is_empty() {
        println!("PREDAJA");
        return;
    }

    availables.sort();

    println!("{}", String::from_utf8(availables).unwrap());
}
