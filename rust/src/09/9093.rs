use std::fmt::Write;
use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut output = String::new();

    for input in buf.lines().skip(1) {
        let words = input.split_whitespace();
        let reversed: Vec<String> = words.map(|word| word.chars().rev().collect()).collect();

        writeln!(output, "{}", reversed.join(" ")).unwrap();
    }

    print!("{output}");
}
