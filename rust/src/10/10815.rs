use std::collections::HashSet;
use std::fmt::Write;
use std::io::{stdin, Read};

fn main() {
    let stdin = stdin();
    let mut stdin = stdin.lock();

    let mut buf = String::new();
    stdin.read_to_string(&mut buf).unwrap();

    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());
    let mut output = String::new();

    let card_set: HashSet<_> = (0..input.next().unwrap())
        .map(|_| input.next().unwrap())
        .collect();

    for num in input.skip(1) {
        write!(output, "{} ", if card_set.contains(&num) { 1 } else { 0 }).unwrap();
    }

    print!("{output}");
}
