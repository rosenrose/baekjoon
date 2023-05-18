use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut counts = [0; 26];
    let offset = b'a';

    for ch in buf.lines().skip(1).map(|input| input.as_bytes()[0]) {
        counts[(ch - offset) as usize] += 1;
    }

    let mut availables: Vec<_> = counts
        .iter()
        .enumerate()
        .filter_map(|(ch, &count)| (count >= 5).then_some(ch as u8 + offset))
        .collect();

    if availables.is_empty() {
        println!("PREDAJA");
        return;
    }

    availables.sort();

    println!("{}", String::from_utf8(availables).unwrap());
}
