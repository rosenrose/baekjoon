use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<f64>().unwrap() / 100.0);
    let mut output = String::new();

    let mut v = 0.0;

    for def in input.skip(1) {
        v = 1.0 - (1.0 - v) * (1.0 - def);

        writeln!(output, "{:.6}", v * 100.0).unwrap();
    }

    print!("{output}");
}
