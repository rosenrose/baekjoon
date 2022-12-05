use std::fmt::Write;
use std::io::{stdin, Read};

fn main() {
    let stdin = stdin();
    let mut stdin = stdin.lock();

    let mut buf = String::new();
    stdin.read_to_string(&mut buf).unwrap();

    let mut output = String::new();

    for line in buf.lines().skip(1) {
        let words = line.split_whitespace();
        let reversed: Vec<String> = words.map(|word| word.chars().rev().collect()).collect();

        writeln!(output, "{}", reversed.join(" ")).unwrap();
    }

    print!("{output}");
}
