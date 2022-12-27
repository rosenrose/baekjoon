use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());
    let mut input = || input.next().unwrap();

    let n = input();
    let max_score = (0..n)
        .map(|_| {
            let (a, d, g) = (input(), input(), input());
            let score = a * (d + g);

            if a == d + g {
                score * 2
            } else {
                score
            }
        })
        .max()
        .unwrap();

    println!("{max_score}");
}
