use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());

    if let [a, b, c, d] = input.collect::<Vec<_>>()[..] {
        println!("{}", (a + d).min(b + c));
    }
}
