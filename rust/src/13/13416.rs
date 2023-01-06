use std::fmt::Write;
use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

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
