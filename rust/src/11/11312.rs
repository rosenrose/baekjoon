use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i64>().unwrap());
    let mut output = String::new();

    for _ in 0..input.next().unwrap() {
        let (a, b) = (input.next().unwrap(), input.next().unwrap());

        writeln!(output, "{}", (a / b) * (a / b)).unwrap();
    }

    print!("{output}");
}
