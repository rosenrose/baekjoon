use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());

    while let (Some(a), Some(b)) = (input.next(), input.next()) {
        if (a, b) == (0, 0) {
            return;
        }

        println!("{}", if a > b { "Yes" } else { "No" })
    }
}
