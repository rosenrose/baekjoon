use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut output = String::new();
    const INVALID: &str = "invalid input";

    for input in buf.lines().skip(1).map(str::trim) {
        if input.is_empty() {
            writeln!(output, "{INVALID}").unwrap();
            continue;
        }
        if input.chars().any(|c| !c.is_ascii_digit()) {
            writeln!(output, "{INVALID}").unwrap();
            continue;
        }

        let num = input.trim_start_matches('0');

        writeln!(output, "{}", if num.is_empty() { "0" } else { num }).unwrap();
    }

    print!("{output}");
}
