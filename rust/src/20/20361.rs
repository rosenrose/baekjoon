use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());
    let mut input = || input.next().unwrap();

    let (_, init, k) = (input(), input(), input());
    let x = (0..k).fold(init, |x, _| match (input(), input()) {
        (a, b) if a == x => b,
        (a, b) if b == x => a,
        _ => x,
    });

    println!("{x}");
}
