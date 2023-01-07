use std::collections::HashSet;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let set: HashSet<_> = buf
        .lines()
        .map(|s| s.parse::<i32>().unwrap() % 42)
        .collect();

    println!("{}", set.len());
}
