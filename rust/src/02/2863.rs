use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());
    let mut input = || input.next().unwrap();

    let (a, b, c, d) = (input(), input(), input(), input());
    let numerator = a * d + b * c;
    let denominators = [c * d, b * d, a * b, a * c];

    let (min_rotate, _) = denominators
        .iter()
        .enumerate()
        .max_by(|(_, &a), (_, &b)| (numerator * b).cmp(&(numerator * a)))
        .unwrap();

    println!("{min_rotate}");
}
