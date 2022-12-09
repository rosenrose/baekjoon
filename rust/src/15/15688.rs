use std::fmt::Write;
use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());
    let mut output = String::new();

    let mut arr: Vec<_> = input.skip(1).collect();
    arr.sort_unstable();

    for num in arr {
        writeln!(output, "{num}").unwrap();
    }

    print!("{output}");
}
