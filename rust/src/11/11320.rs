use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());
    let mut input = || input.next().unwrap();

    for (a, b) in (0..input()).map(|_| (input(), input())) {
        println!("{}", (a / b) * (a / b));
    }
}
