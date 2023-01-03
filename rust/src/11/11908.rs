use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());

    let mut nums: Vec<_> = input.skip(1).collect();
    nums.sort();

    println!("{}", nums.iter().rev().skip(1).sum::<i32>());
}
