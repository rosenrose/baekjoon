use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<f32>().unwrap());
    let mut input = || input.next().unwrap();

    for _ in 0..input() as i32 {
        let (n, d, a, b, f) = (input() as i32, input(), input(), input(), input());

        println!("{n} {:.2}", d / (a + b) * f);
    }
}
