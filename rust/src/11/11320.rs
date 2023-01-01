use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());

    for _ in 0..input.next().unwrap() {
        let (a, b) = (input.next().unwrap(), input.next().unwrap());

        println!("{}", (a / b) * (a / b));
    }
}
