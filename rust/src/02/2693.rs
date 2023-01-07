use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());
    let mut output = String::new();

    for _ in 0..input.next().unwrap() {
        let mut scores: Vec<_> = (0..10).map(|_| input.next().unwrap()).collect();
        scores.sort();

        writeln!(output, "{}", scores.iter().nth_back(2).unwrap()).unwrap();
    }

    print!("{output}");
}
