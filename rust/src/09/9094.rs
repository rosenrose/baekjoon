use std::fmt::Write;
use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());
    let mut output = String::new();

    for _ in 0..input.next().unwrap() {
        let (n, m) = (input.next().unwrap(), input.next().unwrap());
        let mut count = 0;

        for a in 1..n - 1 {
            for b in a + 1..n {
                if (a * a + b * b + m) % (a * b) == 0 {
                    count += 1;
                }
            }
        }

        writeln!(output, "{count}").unwrap();
    }

    print!("{output}");
}
