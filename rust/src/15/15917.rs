use std::fmt::Write;
use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());
    let mut output = String::new();

    for num in input.skip(1) {
        writeln!(output, "{}", if num & -num == num { 1 } else { 0 }).unwrap();
    }

    print!("{output}");
}
