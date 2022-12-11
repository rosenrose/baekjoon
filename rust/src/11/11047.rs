use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());
    input.next();

    let mut k = input.next().unwrap();
    let mut count = 0;

    for coin in input.rev() {
        count += k / coin;
        k %= coin;
    }

    println!("{count}");
}
