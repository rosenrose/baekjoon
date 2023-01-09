use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());
    let mut input = || input.next().unwrap();

    for _ in 0..input() {
        let (n, d) = (input(), input());
        let count = (0..n)
            .filter(|_| {
                let (v, f, c) = (input(), input(), input());
                v * f / c >= d
            })
            .count();

        println!("{count}");
    }
}
