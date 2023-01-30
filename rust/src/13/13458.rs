use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i64>);

    let (c, b) = (input.next_back().unwrap(), input.next_back().unwrap());

    let watchers: i64 = input
        .skip(1)
        .map(|count| 1 + ((count - b).max(0) as f64 / c as f64).ceil() as i64)
        .sum();

    println!("{watchers}");
}
