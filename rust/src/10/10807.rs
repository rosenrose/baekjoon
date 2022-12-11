use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());
    input.next();

    let v = input.next_back().unwrap();
    let count = input.filter(|&n| n == v).count();

    println!("{count}");
}
