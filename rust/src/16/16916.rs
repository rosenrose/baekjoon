use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut input = buf.split_ascii_whitespace();
    let (s, p) = (input.next().unwrap(), input.next().unwrap());

    println!("{}", if s.contains(p) { 1 } else { 0 });
}
