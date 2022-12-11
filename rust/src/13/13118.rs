use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());

    let points: Vec<_> = (0..4).map(|_| input.next().unwrap()).collect();
    let x = input.next().unwrap();

    let number = match points.iter().position(|&p| p == x) {
        Some(i) => i + 1,
        None => 0,
    };

    println!("{number}");
}
