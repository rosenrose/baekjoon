use std::io::{self, Read};

fn main() {
    let mut buf = Vec::new();
    io::stdin().read_to_end(&mut buf).unwrap();

    const N: i32 = 5_000_000;
    let sum: i64 = buf
        .split(|&ch| ch == b'\n')
        .skip(1)
        .map(|input| {
            input
                .iter()
                .fold(0, |acc, &ch| acc * 10 + (ch - b'0') as i64)
        })
        .sum();

    println!("{N}\n{sum}");
}
