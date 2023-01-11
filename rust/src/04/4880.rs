use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());
    let mut output = String::new();

    while let (Some(a), Some(b), Some(c)) = (input.next(), input.next(), input.next()) {
        if (a, b, c) == (0, 0, 0) {
            break;
        }

        (match c - b {
            diff if diff == b - a => writeln!(output, "AP {}", c + diff),
            _ => writeln!(output, "GP {}", c * (c / b)),
        })
        .unwrap()
    }

    print!("{output}");
}
