use std::collections::HashSet;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let letters: Vec<_> = buf
        .lines()
        .skip(1)
        .map(|s| s.chars().nth(0).unwrap())
        .collect();
    let char_set: HashSet<_> = letters.iter().collect();

    let mut availables: Vec<_> = char_set
        .into_iter()
        .filter(|ch| letters.iter().filter(|letter| letter == ch).count() >= 5)
        .collect();

    if availables.is_empty() {
        println!("PREDAJA");
        return;
    }

    availables.sort();

    println!("{}", String::from_iter(availables));
}
