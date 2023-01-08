use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());

    let n = input.next().unwrap();
    let sum: i32 = input.skip(1).map(|num| (num + 1) / 2).sum();

    println!("{}", if sum >= n { "YES" } else { "NO" });
}
