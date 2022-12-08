use std::collections::HashSet;
use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

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

    if availables.len() == 0 {
        println!("PREDAJA");
        return;
    }

    availables.sort();

    println!("{}", String::from_iter(availables));
}
