use std::fmt::Write;
use std::io::{stdin, Read};

fn main() {
    let stdin = stdin();
    let mut stdin = stdin.lock();

    let mut buf = String::new();
    stdin.read_to_string(&mut buf).unwrap();

    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());
    let mut output = String::new();

    let mut coords: Vec<_> = (0..input.next().unwrap())
        .map(|_| (input.next().unwrap(), input.next().unwrap()))
        .collect();

    coords.sort_unstable_by(
        |(x1, y1), (x2, y2)| {
            if x1 == x2 {
                y1.cmp(y2)
            } else {
                x1.cmp(x2)
            }
        },
    );

    for (x, y) in coords {
        writeln!(output, "{x} {y}").unwrap();
    }

    print!("{output}");
}
