use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<f32>().unwrap());
    let mut input = || input.next().unwrap();

    let (k, d1, d2) = (input(), input(), input());
    let d3 = (d1 - d2) / 2.0;

    println!("{}", k * k - d3 * d3);
}
