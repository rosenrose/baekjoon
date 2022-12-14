use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());
    let mut output = String::new();

    let mut coords: Vec<_> = (0..input.next().unwrap())
        .map(|_| {
            let (x, y) = (input.next().unwrap(), input.next().unwrap());
            (y, x)
        })
        .collect();

    coords.sort_unstable();

    for (y, x) in coords {
        writeln!(output, "{x} {y}").unwrap();
    }

    print!("{output}");
}
