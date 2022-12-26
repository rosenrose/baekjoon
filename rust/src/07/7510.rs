use std::fmt::Write;
use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i64>().unwrap());
    let mut output = String::new();

    for i in 1..=input.next().unwrap() {
        let (a, b, c) = (
            input.next().unwrap(),
            input.next().unwrap(),
            input.next().unwrap(),
        );

        let is_right = match a.max(b).max(c) {
            longest if longest == a => a * a == b * b + c * c,
            longest if longest == b => b * b == a * a + c * c,
            longest if longest == c => c * c == a * a + b * b,
            _ => false,
        };

        writeln!(output, "Scenario #{i}:").unwrap();
        writeln!(output, "{}\n", if is_right { "yes" } else { "no" }).unwrap();
    }

    print!("{output}");
}
