use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut input = || input.next().unwrap();
    let mut output = String::new();

    let mut coords: Vec<_> = (0..input()).map(|_| (input(), input())).collect();
    coords.sort_unstable_by_key(|&(x, y)| (y, x));

    for (x, y) in coords {
        writeln!(output, "{x} {y}").unwrap();
    }

    print!("{output}");
}
