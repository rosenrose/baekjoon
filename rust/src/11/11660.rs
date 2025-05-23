use std::fmt::Write;
use std::io;

const MAX: usize = 1024 + 1;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let mut input = || input.next().unwrap();
    let mut output = String::new();

    let [n, m] = [(); 2].map(|_| input());
    let mut sum_accum = [[0; MAX]; MAX];

    for i in 1..=n {
        for j in 1..=n {
            sum_accum[i][j] = sum_accum[i][j - 1] + input() as i32;
        }
        for j in 1..=n {
            sum_accum[i][j] += sum_accum[i - 1][j];
        }
    }
    // println!("{sum_accum:?}");
    for _ in 0..m {
        let [x1, y1, x2, y2] = [(); 4].map(|_| input());
        let (x1, y1, x2, y2) = (y1 - 1, x1 - 1, y2, x2);
        let sum = sum_accum[y2][x2] - sum_accum[y2][x1] - sum_accum[y1][x2] + sum_accum[y1][x1];

        writeln!(output, "{sum}").unwrap();
    }

    print!("{output}");
}
