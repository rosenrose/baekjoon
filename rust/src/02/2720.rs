use std::fmt::Write;
use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());
    let mut output = String::new();

    for mut cents in input.skip(1) {
        let mut counts = [0, 0, 0, 0];
        let values = [25, 10, 5, 1];

        for (i, value) in counts.iter_mut().enumerate() {
            *value += cents / values[i];
            cents %= values[i];
        }

        writeln!(
            output,
            "{} {} {} {}",
            counts[0], counts[1], counts[2], counts[3]
        )
        .unwrap();
    }

    print!("{output}");
}
