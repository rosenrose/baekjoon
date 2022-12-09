use std::fmt::Write;
use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<usize>().unwrap());
    let mut output = String::new();

    let (n, m) = (input.next().unwrap(), input.next().unwrap());
    let sum_accum = (0..n).fold(vec![0], |mut acc, _| {
        acc.push(acc.last().unwrap() + input.next().unwrap());
        acc
    });

    for _ in 0..m {
        let (i, j) = (input.next().unwrap(), input.next().unwrap());
        let sum = sum_accum[j] - sum_accum[i - 1];

        writeln!(output, "{sum}").unwrap();
    }

    print!("{output}");
}
