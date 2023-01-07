use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());
    let mut output = String::new();

    while let Some(a) = input.next() {
        writeln!(output, "{}", a + input.next().unwrap()).unwrap();
    }

    print!("{output}");
}
