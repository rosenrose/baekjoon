use std::collections::HashSet;
use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());

    let seats: Vec<_> = input.skip(1).collect();
    let denied: HashSet<_> = seats.iter().collect();

    println!("{}", seats.len() - denied.len());
}
