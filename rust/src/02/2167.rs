use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut input = || input.next().unwrap();
    let mut output = String::new();

    let (n, m) = (input() as usize, input() as usize);
    let mut sum_accum = vec![vec![0; m + 1]; n + 1];

    for i in 1..=n {
        for j in 1..=m {
            sum_accum[i][j] = sum_accum[i][j - 1] + input();
        }
        for j in 1..=m {
            sum_accum[i][j] += sum_accum[i - 1][j];
        }
    }

    for _ in 0..input() {
        let (i, j, x, y) = (
            input() as usize,
            input() as usize,
            input() as usize,
            input() as usize,
        );

        let (i, j) = (i - 1, j - 1);
        let sum = sum_accum[x][y] - sum_accum[i][y] - sum_accum[x][j] + sum_accum[i][j];

        writeln!(output, "{sum}").unwrap();
    }

    print!("{output}");
}
