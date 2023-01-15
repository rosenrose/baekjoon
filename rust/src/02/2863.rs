use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
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
