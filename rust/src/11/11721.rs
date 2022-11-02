use std::{io, str};

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();

    let words = buf
        .trim()
        .as_bytes()
        .chunks(10)
        .map(|s| str::from_utf8(s).unwrap());

    for word in words {
        println!("{word}");
    }
}
