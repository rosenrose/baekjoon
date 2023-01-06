use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i64>().unwrap());

    let n = input.next().unwrap();
    let sum: i64 = (0..n)
        .map(|_| (0..n).map(|_| input.next().unwrap()).sum::<i64>())
        .sum();

    println!("{sum}");
}
