use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());

    let (x, y) = (input.next().unwrap(), input.next().unwrap());
    let mut price = x as f64 * 1000.0 / y as f64;

    let n = input.next().unwrap();

    for _ in 0..n {
        let (x, y) = (input.next().unwrap(), input.next().unwrap());
        price = price.min(x as f64 * 1000.0 / y as f64);
    }

    println!("{price:.2}");
}
