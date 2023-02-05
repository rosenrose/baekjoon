use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i64>);

    let cluster = input.next_back().unwrap();
    let count: i64 = (0..input.next().unwrap())
        .map(|_| (input.next().unwrap() as f64 / cluster as f64).ceil() as i64)
        .sum();

    println!("{}", cluster * count);
}
