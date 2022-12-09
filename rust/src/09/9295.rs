use std::fmt::Write;
use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());
    let mut output = String::new();

    for i in 1..=input.next().unwrap() {
        let sum = input.next().unwrap() + input.next().unwrap();

        writeln!(output, "Case {i}: {sum}").unwrap();
    }

    print!("{output}");
}
