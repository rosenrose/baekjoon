use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());
    let mut input = || input.next().unwrap();

    while let (a @ 1.., b @ 1.., c @ 1..) = (input(), input(), input()) {
        let is_right = match a.max(b).max(c) {
            longest if longest == a => a * a == b * b + c * c,
            longest if longest == b => b * b == a * a + c * c,
            longest if longest == c => c * c == a * a + b * b,
            _ => false,
        };

        println!("{}", if is_right { "right" } else { "wrong" });
    }
}
