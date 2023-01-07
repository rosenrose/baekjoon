use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());

    while let (Some(a), Some(b), Some(c)) = (input.next(), input.next(), input.next()) {
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
