use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i64>);
    let mut input = || input.next().unwrap();
    let mut output = String::new();

    for (a, b) in (0..input()).map(|_| (input(), input())) {
        writeln!(output, "{}", (a / b) * (a / b)).unwrap();
    }

    print!("{output}");
}
