use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let sum: i32 = buf
        .lines()
        .next_back()
        .unwrap()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i32)
        .sum();

    println!("{sum}");
}
