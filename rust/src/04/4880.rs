use std::fmt::Write;
use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());
    let mut output = String::new();

    loop {
        let (a, b, c) = (
            input.next().unwrap(),
            input.next().unwrap(),
            input.next().unwrap(),
        );

        match c - b {
            0 => break,
            diff if diff == b - a => writeln!(output, "AP {}", c + diff).unwrap(),
            _ => writeln!(output, "GP {}", c * (c / b)).unwrap(),
        }
    }

    print!("{output}");
}
