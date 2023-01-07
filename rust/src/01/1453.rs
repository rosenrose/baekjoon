use std::collections::HashSet;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());

    let seats: Vec<_> = input.skip(1).collect();
    let denied: HashSet<_> = seats.iter().collect();

    println!("{}", seats.len() - denied.len());
}
