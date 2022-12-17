use std::fmt::Write;
use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());
    let mut output = String::new();

    let sum_accum = (0..input.next().unwrap()).fold(vec![0], |mut acc, _| {
        acc.push(*acc.last().unwrap() + input.next().unwrap());
        acc
    });

    for _ in 0..input.next().unwrap() {
        let (i, j) = (
            input.next().unwrap() as usize,
            input.next().unwrap() as usize,
        );

        writeln!(output, "{}", sum_accum[j] - sum_accum[i - 1]).unwrap();
    }

    print!("{output}");
}
