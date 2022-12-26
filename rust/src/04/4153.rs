use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());

    loop {
        let (a, b, c) = (
            input.next().unwrap(),
            input.next().unwrap(),
            input.next().unwrap(),
        );

        if (a, b, c) == (0, 0, 0) {
            return;
        }

        let is_right = match a.max(b).max(c) {
            longest if longest == a => a * a == b * b + c * c,
            longest if longest == b => b * b == a * a + c * c,
            longest if longest == c => c * c == a * a + b * b,
            _ => false,
        };

        println!("{}", if is_right { "right" } else { "wrong" });
    }
}
