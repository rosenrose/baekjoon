use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());
    let mut input = || input.next().unwrap();
    let mut output = String::new();

    for (n, m) in (0..input()).map(|_| (input(), input())) {
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
