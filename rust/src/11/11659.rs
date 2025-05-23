use std::fmt::Write;
use std::io;

const MAX: usize = 100_000 + 1;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let mut input = || input.next().unwrap();
    let mut output = String::new();

    let [n, m] = [(); 2].map(|_| input());
    let mut sum_accum = [0; MAX];

    for i in 1..=n {
        sum_accum[i] = sum_accum[i - 1] + input() as i32;
    }

    for [i, j] in (0..m).map(|_| [(); 2].map(|_| input())) {
        writeln!(output, "{}", sum_accum[j] - sum_accum[i - 1]).unwrap();
    }

    print!("{output}");
}
