use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i64>);
    let mut input = || input.next().unwrap();
    let mut output = String::new();

    let sum = |num: i64| (num * (num + 1)) / 2;

    for (i, (n, m)) in (1..=input()).map(|i| (i, (input(), input()))) {
        writeln!(output, "Scenario #{i}:").unwrap();
        writeln!(output, "{}\n", sum(m) - sum(n - 1)).unwrap();
    }

    print!("{output}");
}
