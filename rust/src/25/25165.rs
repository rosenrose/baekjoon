use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());
    let mut input = || input.next().unwrap();

    let (n, _, _, d, r, _) = (input(), input(), input(), input(), input(), input());

    println!(
        "{}",
        match ((n % 2, d), r) {
            ((0, 0) | (1, 1), _) => "NO...",
            ((0, 1) | (1, 0), r) if r < n => "NO...",
            _ => "YES!",
        }
    );
}
