use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());
    let mut input = || input.next().unwrap();

    let (x, y) = (input(), input());
    let mut price = x as f64 * 1000.0 / y as f64;

    let n = input();

    for _ in 0..n {
        let (x, y) = (input(), input());
        price = price.min(x as f64 * 1000.0 / y as f64);
    }

    println!("{price:.2}");
}
