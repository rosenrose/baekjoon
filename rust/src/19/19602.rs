use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let score: usize = input.enumerate().map(|(i, n)| (i + 1) * n).sum();

    println!("{}", if score >= 10 { "happy" } else { "sad" });
}
