use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());

    let (a, b, c) = (
        input.next().unwrap(),
        input.next().unwrap(),
        input.next().unwrap(),
    );
    let balance = a + b;
    let price = c * 2;

    println!(
        "{}",
        if balance < price {
            balance
        } else {
            balance - price
        }
    );
}
