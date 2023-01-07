use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());
    let mut input = || input.next().unwrap();
    let mut output = String::new();

    let sum_accum = (0..input()).fold(vec![0], |mut acc, _| {
        acc.push(acc.last().unwrap() + input());
        acc
    });

    for _ in 0..input() {
        let (i, j) = (input() as usize, input() as usize);

        writeln!(output, "{}", sum_accum[j] - sum_accum[i - 1]).unwrap();
    }

    print!("{output}");
}
