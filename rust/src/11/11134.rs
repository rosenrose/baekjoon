use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut input = || input.next().unwrap();

    for (n, c) in (0..input()).map(|_| (input(), input())) {
        println!("{}", (n as f64 / c as f64).ceil());
    }
}
