use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());
    input.next();

    let mut k = input.next().unwrap();
    let count = input.rev().fold(0, |acc, coin| {
        let count = acc + (k / coin);
        k %= coin;

        count
    });

    println!("{count}");
}
