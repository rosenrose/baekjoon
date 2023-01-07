use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<f32>().unwrap());
    let mut input = || input.next().unwrap();

    let (a, b, c, m) = (input(), input(), input(), input());
    let n = b - m;
    let x = (a * a - c * c + n * n - m * m) / (2.0 * (m + n));

    println!("{}", a * a - m * m - 2.0 * m * x);
}
