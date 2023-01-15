use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i128>);
    let mut input = || input.next().unwrap();
    let mut output = String::new();

    for (n, m) in (0..input()).map(|_| (input(), input())) {
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
