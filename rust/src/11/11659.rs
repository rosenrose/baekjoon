use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<usize>().unwrap());
    let mut input = || input.next().unwrap();
    let mut output = String::new();

    let (n, m) = (input(), input());
    let sum_accum = (0..n).fold(vec![0], |mut acc, _| {
        acc.push(acc.last().unwrap() + input());
        acc
    });

    for _ in 0..m {
        let (i, j) = (input(), input());
        let sum = sum_accum[j] - sum_accum[i - 1];

        writeln!(output, "{sum}").unwrap();
    }

    print!("{output}");
}
