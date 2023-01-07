use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut output = String::new();

    for input in buf.lines().skip(1) {
        let words = input.split_whitespace();
        let reversed: Vec<String> = words.map(|word| word.chars().rev().collect()).collect();

        writeln!(output, "{}", reversed.join(" ")).unwrap();
    }

    print!("{output}");
}
