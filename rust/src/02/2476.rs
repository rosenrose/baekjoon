use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());

    let max_prize = (0..input.next().unwrap())
        .map(|_| {
            let (a, b, c) = (
                input.next().unwrap(),
                input.next().unwrap(),
                input.next().unwrap(),
            );

            match (a, b, c) {
                (a, b, c) if a == b && b == c => 10000 + a * 1000,
                (a, b, _) if a == b => 1000 + a * 100,
                (a, _, b) if a == b => 1000 + a * 100,
                (_, a, b) if a == b => 1000 + a * 100,
                _ => a.max(b).max(c) * 100,
            }
        })
        .max()
        .unwrap();

    println!("{max_prize}");
}
