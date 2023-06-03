use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();
    let mut input = || input.next().unwrap();

    let n: i32 = input().parse().unwrap();
    let (easy, _) = (0..n)
        .map(|_| (input(), input()))
        .min_by_key(|&(_, difficulty)| difficulty)
        .unwrap();

    println!("{easy}");
}
