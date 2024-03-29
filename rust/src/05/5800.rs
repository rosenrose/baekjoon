use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut input = || input.next().unwrap();
    let mut output = String::new();

    for i in 1..=input() {
        let mut scores: Vec<_> = (0..input()).map(|_| input()).collect();
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
