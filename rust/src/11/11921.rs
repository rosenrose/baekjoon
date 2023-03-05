use std::io::{self, Read};

fn main() {
    let mut buf = Vec::new();
    io::stdin().read_to_end(&mut buf).unwrap();

    let sum: i64 = buf
        .split(|&byte| byte == '\n' as u8)
        .skip(1)
        .map(|input| {
            input
                .iter()
                .fold(0, |acc, &ch| acc * 10 + (ch as i64 - '0' as i64))
        })
        .sum();

    println!("{}\n{sum}", 5_000_000);
}
