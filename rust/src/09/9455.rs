use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let mut input = || input.next().unwrap();
    let mut output = String::new();

    for _ in 0..input() {
        let (m, n) = (input(), input());
        let grid: Vec<Vec<_>> = (0..m).map(|_| (0..n).map(|_| input()).collect()).collect();

        let mut sum = 0;

        for c in 0..n {
            let mut count_1 = 0;

            for r in (0..m).rev() {
                if grid[r][c] == 1 {
                    sum += (m - 1 - count_1) - r;
                    count_1 += 1;
                }
            }
        }

        writeln!(output, "{sum}").unwrap();
    }

    print!("{output}");
}
