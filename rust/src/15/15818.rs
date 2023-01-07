use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i64>().unwrap());
    input.next();

    let m = input.next().unwrap();
    let remainder = input.fold(1, |acc, num| (acc * (num % m)) % m);

    println!("{remainder}");
}
