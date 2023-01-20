use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut input = || input.next().unwrap();
    let mut output = String::new();

    let (n, m) = (input() as usize, input() as usize);

    let sum_accum = (0..n).fold(vec![vec![0; m + 1]], |mut acc, _| {
        let mut row_accum = vec![0];
        row_accum.extend((0..m).scan(0, |acc, _| {
            *acc += input();
            Some(*acc)
        }));

        let row: Vec<_> = acc
            .last()
            .unwrap()
            .iter()
            .zip(row_accum)
            .map(|(col1, col2)| col1 + col2)
            .collect();

        acc.push(row);
        acc
    });

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
