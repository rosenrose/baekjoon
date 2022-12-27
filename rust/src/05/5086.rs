use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());

    while let (Some(a), Some(b)) = (input.next(), input.next()) {
        match (a, b) {
            (0, 0) => return,
            (a, b) if a % b == 0 => println!("multiple"),
            (a, b) if b % a == 0 => println!("factor"),
            _ => println!("neither"),
        };
    }
}
