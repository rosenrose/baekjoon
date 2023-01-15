use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut input = || input.next().unwrap();

    let mut price = input() as f64 * 1000.0 / input() as f64;

    for (x, y) in (0..input()).map(|_| (input(), input())) {
        price = price.min(x as f64 * 1000.0 / y as f64);
    }

    println!("{price:.2}");
}
