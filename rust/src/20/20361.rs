use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());
    let mut input = || input.next().unwrap();

    let (_, mut x, k) = (input(), input(), input());

    for _ in 0..k {
        x = match (input(), input()) {
            (a, b) if a == x => b,
            (a, b) if b == x => a,
            _ => x,
        }
    }

    println!("{x}");
}
