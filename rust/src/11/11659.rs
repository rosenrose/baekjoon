use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut input = || input.next().unwrap();
    let mut output = String::new();

    let (n, m) = (input(), input());
    let sum_accum: Vec<_> = [0]
        .into_iter()
        .chain((0..n).scan(0, |acc, _| {
            *acc += input();
            Some(*acc)
        }))
        .collect();

    for (i, j) in (0..m).map(|_| (input() as usize, input() as usize)) {
        writeln!(output, "{}", sum_accum[j] - sum_accum[i - 1]).unwrap();
    }

    print!("{output}");
}
