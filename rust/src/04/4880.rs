use std::fmt::Write;
use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());
    let mut output = String::new();

    while let (Some(a), Some(b), Some(c)) = (input.next(), input.next(), input.next()) {
        match c - b {
            0 => break,
            diff if diff == b - a => writeln!(output, "AP {}", c + diff).unwrap(),
            _ => writeln!(output, "GP {}", c * (c / b)).unwrap(),
        }
    }

    print!("{output}");
}
