use std::fmt::Write;
use std::io::{stdin, Read};

fn main() {
    let stdin = stdin();
    let mut stdin = stdin.lock();

    let mut buf = String::new();
    stdin.read_to_string(&mut buf).unwrap();

    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());
    let mut output = String::new();

    let (n, m) = (input.next().unwrap(), input.next().unwrap());
    let sum_arr: Vec<Vec<_>> = (0..n)
        .map(|_| {
            (0..m).fold(vec![0], |mut row, _| {
                row.push(row.last().unwrap() + input.next().unwrap());
                row
            })
        })
        .collect();

    for _ in 0..input.next().unwrap() {
        let (i, j, x, y) = (
            input.next().unwrap() as usize,
            input.next().unwrap() as usize,
            input.next().unwrap() as usize,
            input.next().unwrap() as usize,
        );
        let (i, j) = (i - 1, j - 1);

        let sum: i32 = sum_arr[i..x].iter().map(|row| row[y] - row[j]).sum();

        writeln!(output, "{sum}").unwrap();
    }

    print!("{output}");
}
