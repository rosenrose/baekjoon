use std::{io, str};

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();

    let bin = buf.trim();

    let oct: String = bin
        .as_bytes()
        .rchunks(3)
        .rev()
        .map(|s| {
            let chunk = str::from_utf8(s).unwrap();
            let digit = u32::from_str_radix(chunk, 2).unwrap();

            char::from_digit(digit, 8).unwrap()
        })
        .collect();

    println!("{oct}");
}
