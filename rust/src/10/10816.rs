use std::collections::HashMap;
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

    let mut counts = HashMap::new();
    let n = input.next().unwrap();

    for _ in 0..n {
        counts
            .entry(input.next().unwrap())
            .and_modify(|c| *c += 1)
            .or_insert(1);
    }

    for num in input.skip(1) {
        write!(output, "{} ", counts.get(&num).unwrap_or(&0)).unwrap();
    }

    print!("{output}");
}
