use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());

    while let (Some(a @ 1..), Some(b @ 1..)) = (input.next(), input.next()) {
        println!("{}", if a > b { "Yes" } else { "No" })
    }
}
