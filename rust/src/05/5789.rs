use std::fmt::Write;
use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let input = buf.lines().map(|s| s.as_bytes());
    let mut output = String::new();

    for num in input.skip(1) {
        let half = num.len() / 2;

        writeln!(
            output,
            "{}",
            if num[half - 1] == num[half] {
                "Do-it"
            } else {
                "Do-it-Not"
            }
        )
        .unwrap();
    }

    print!("{output}");
}
