use std::fmt::Write;
use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());
    let mut output = String::new();

    let (n, m) = (
        input.next().unwrap() as usize,
        input.next().unwrap() as usize,
    );

    let sum_accum = (0..n).fold(vec![vec![0; m + 1]], |mut acc, _| {
        let row_accum = (0..m).fold(vec![0], |mut acc, _| {
            acc.push(acc.last().unwrap() + input.next().unwrap());
            acc
        });

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

    for _ in 0..input.next().unwrap() {
        if let [i, j, x, y] = (0..4)
            .map(|_| input.next().unwrap() as usize)
            .collect::<Vec<_>>()[..]
        {
            let (i, j) = (i - 1, j - 1);
            let sum = sum_accum[x][y] - sum_accum[i][y] - sum_accum[x][j] + sum_accum[i][j];

            writeln!(output, "{sum}").unwrap();
        }
    }

    print!("{output}");
}
