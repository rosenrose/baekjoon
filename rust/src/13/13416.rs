use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());
    let mut input = || input.next().unwrap();
    let mut output = String::new();

    for _ in 0..input() {
        let profit: i32 = (0..input())
            .map(|_| (0..3).map(|_| input()).max().unwrap().max(0))
            .sum();

        writeln!(output, "{profit}").unwrap();
    }

    print!("{output}");
}
