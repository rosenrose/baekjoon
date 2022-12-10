use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());

    let s: i32 = (0..4).map(|_| input.next().unwrap()).sum();
    let t: i32 = input.sum();

    println!("{}", s.max(t));
}
