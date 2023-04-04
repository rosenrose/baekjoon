use std::collections::HashSet;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let input: Vec<_> = buf.lines().skip(2).collect();

    let count: usize = input
        .split(|&s| s == "ENTER")
        .map(|names| HashSet::<&&str>::from_iter(names).len())
        .sum();

    println!("{count}");
}
