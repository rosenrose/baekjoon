use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();

    let mbti = input.next().unwrap();
    let count = input.skip(1).filter(|&friend| friend == mbti).count();

    println!("{count}");
}
