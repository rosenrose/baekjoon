use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<f64>().unwrap());

    let (d1, d2) = (input.next().unwrap(), input.next().unwrap());

    println!("{:.6}", (d1 * 2.0) + (2.0 * 3.141592 * d2));
}
