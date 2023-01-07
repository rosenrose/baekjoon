use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());
    let mut output = String::new();

    for num in input.skip(1) {
        writeln!(output, "{}", if num & -num == num { 1 } else { 0 }).unwrap();
    }

    print!("{output}");
}
