use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let s: i32 = input.by_ref().take(4).sum();
    let t: i32 = input.sum();

    println!("{}", s.max(t));
}
