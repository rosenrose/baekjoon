use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.lines();

    let s: String = input
        .next()
        .unwrap()
        .chars()
        .filter(|c| c.is_alphabetic())
        .collect();
    let k = input.next().unwrap();

    println!("{}", if s.contains(k) { 1 } else { 0 });
}
