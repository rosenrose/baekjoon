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
        let n = input.next().unwrap();

        let mut scores: Vec<_> = (0..n).map(|_| input.next().unwrap()).collect();
        scores.sort_unstable();

        let min = scores[0];
        let max = scores.last().unwrap();
        let largest_gap = (1..scores.len())
            .map(|j| scores[j - 1].abs_diff(scores[j]))
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
