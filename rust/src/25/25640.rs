use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut input = buf.split_ascii_whitespace();

    let mbti = input.next().unwrap();
    let count = input.skip(1).filter(|&friend| friend == mbti).count();

    println!("{count}");
}
