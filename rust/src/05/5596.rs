use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());

    let s: i32 = (0..4).map(|_| input.next().unwrap()).sum();
    let t: i32 = input.sum();

    println!("{}", s.max(t));
}
