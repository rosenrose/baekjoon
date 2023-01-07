use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());
    let mut input = || input.next().unwrap();

    let (x, n) = (input(), input());
    let sum = (0..n).fold(0, |acc, _| acc + (input() * input()));

    println!("{}", if sum == x { "Yes" } else { "No" });
}
