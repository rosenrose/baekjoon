use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());

    for _ in 0..input.next().unwrap() {
        println!("{}", (0..3).map(|_| input.next().unwrap()).min().unwrap());
    }
}
