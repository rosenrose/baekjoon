use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());

    while let (Some(a), Some(b)) = (input.next(), input.next()) {
        if (a, b) == (0, 0) {
            return;
        }

        println!("{}", (2 * a - b).min(2 * b - a).min((a + b) / 2));
    }
}
