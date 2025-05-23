use std::fmt::Write;
use std::io;

const MAX: usize = 100_000 + 1;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut input = || input.next().unwrap();
    let mut output = String::new();

    let n = input() as usize;
    let mut sum_accum = [0; MAX];

    for i in 1..=n {
        sum_accum[i] = sum_accum[i - 1] + input();
    }

    for (i, j) in (0..input()).map(|_| (input() as usize, input() as usize)) {
        writeln!(output, "{}", sum_accum[j] - sum_accum[i - 1]).unwrap();
    }

    print!("{output}");
}
