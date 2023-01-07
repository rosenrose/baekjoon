use std::collections::HashSet;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let names: Vec<_> = buf
        .lines()
        .skip(1)
        .map(|s| s.chars().nth(0).unwrap())
        .collect();
    let letters: HashSet<_> = names.iter().collect();

    let mut availables: Vec<_> = letters
        .into_iter()
        .filter(|ch| names.iter().filter(|name| name == ch).count() >= 5)
        .collect();

    if availables.is_empty() {
        println!("PREDAJA");
        return;
    }

    availables.sort();

    println!("{}", String::from_iter(availables));
}
