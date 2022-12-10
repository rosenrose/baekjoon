use std::collections::HashSet;
use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let set: HashSet<_> = buf
        .lines()
        .map(|s| s.parse::<i32>().unwrap() % 42)
        .collect();

    println!("{}", set.len());
}
