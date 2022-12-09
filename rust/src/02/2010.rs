use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let count = buf
        .split_ascii_whitespace()
        .skip(1)
        .map(|s| s.parse::<i32>().unwrap() - 1)
        .sum::<i32>()
        + 1;

    print!("{count}");
}
