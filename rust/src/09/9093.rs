use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut output = String::new();

    for input in buf.lines().skip(1) {
        let reversed: Vec<String> = input
            .split(' ')
            .map(|word| word.chars().rev().collect())
            .collect();

        writeln!(output, "{}", reversed.join(" ")).unwrap();
    }

    print!("{output}");
}
