use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());
    let mut input = || input.next().unwrap();

    let (a, b, c) = (input(), input(), input());
    let n = input();

    let max_score = (0..n)
        .map(|_| {
            (0..3)
                .map(|_| a * input() + b * input() + c * input())
                .sum::<i32>()
        })
        .max()
        .unwrap();

    println!("{max_score}")
}
