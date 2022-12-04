use std::io::{stdin, Read};

fn main() {
    let stdin = stdin();
    let mut stdin = stdin.lock();

    let mut buf = String::new();
    stdin.read_to_string(&mut buf).unwrap();

    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i64>().unwrap());
    input.next();

    let (c, b) = (input.next_back().unwrap(), input.next_back().unwrap());
    let watchers = input.map(|count| 1 + ((count - b).max(0) as f64 / c as f64).ceil() as i64);

    println!("{}", watchers.sum::<i64>());
}
