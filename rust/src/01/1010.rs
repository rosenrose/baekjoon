use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i128>().unwrap());
    let mut output = String::new();

    for _ in 0..input.next().unwrap() {
        let (n, m) = (input.next().unwrap(), input.next().unwrap());

        writeln!(output, "{}", combination_num(m, n)).unwrap();
    }

    print!("{output}");
}

fn combination_num(n: i128, r: i128) -> i128 {
    if n == r || r == 0 {
        return 1;
    }

    (n - r + 1..=n).product::<i128>() / (1..=r).product::<i128>()
}
