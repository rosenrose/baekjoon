use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());

    for _ in 0..input.next().unwrap() {
        let (n, c) = (input.next().unwrap(), input.next().unwrap());

        println!("{}", (n as f64 / c as f64).ceil());
    }
}
