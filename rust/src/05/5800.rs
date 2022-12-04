use std::fmt::Write;
use std::io::{stdin, Read};

fn main() {
    let stdin = stdin();
    let mut stdin = stdin.lock();

    let mut buf = String::new();
    stdin.read_to_string(&mut buf).unwrap();

    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());
    let mut output = String::new();

    for i in 1..=input.next().unwrap() {
        let n = input.next().unwrap();

        let mut scores: Vec<_> = (0..n).map(|_| input.next().unwrap()).collect();
        scores.sort_unstable();

        let min = scores[0];
        let max = scores.last().unwrap();
        let largest_gap = (0..scores.len() - 1)
            .map(|j| scores[j].abs_diff(scores[j + 1]))
            .max()
            .unwrap();

        writeln!(
            output,
            "Class {i}\nMax {max}, Min {min}, Largest gap {largest_gap}"
        )
        .unwrap();
    }

    print!("{output}");
}
