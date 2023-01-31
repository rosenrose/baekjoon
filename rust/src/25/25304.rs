use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut input = || input.next().unwrap();

    let (x, n) = (input(), input());
    let sum: i32 = (0..n).map(|_| input() * input()).sum();

    println!("{}", if sum == x { "Yes" } else { "No" });
}
