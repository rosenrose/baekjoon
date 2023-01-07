use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());
    let mut input = || input.next().unwrap();
    let mut output = String::new();

    let (n, m) = (input() as usize, input());

    let sum_accum: Vec<_> = (0..n).fold(vec![vec![0; n + 1]], |mut acc, _| {
        let row_accum = (0..n).fold(vec![0], |mut acc, _| {
            acc.push(acc.last().unwrap() + input());
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
    // println!("{sum_accum:?}");
    for _ in 0..m {
        let (x1, y1, x2, y2) = (
            input() as usize,
            input() as usize,
            input() as usize,
            input() as usize,
        );

        let (x1, y1, x2, y2) = (y1 - 1, x1 - 1, y2, x2);
        let sum = sum_accum[y2][x2] - sum_accum[y2][x1] - sum_accum[y1][x2] + sum_accum[y1][x1];

        writeln!(output, "{sum}").unwrap();
    }

    print!("{output}");
}

// let mut sum = 0;
// let row: Vec<_> = acc
//     .last()
//     .unwrap()
//     .iter()
//     .skip(1)
//     .fold(vec![0], |mut row_acc, col| {
//         sum += input.next().unwrap();
//         row_acc.push(col + sum);
//         row_acc
//     });
