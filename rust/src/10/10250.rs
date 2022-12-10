use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());

    for _ in 0..input.next().unwrap() {
        let (h, _, n) = (
            input.next().unwrap(),
            input.next().unwrap(),
            input.next().unwrap(),
        );

        let floor = if n % h == 0 { h } else { n % h };
        let num_from_elevator = (n as f64 / h as f64).ceil() as i32;

        println!("{floor}{num_from_elevator:02}");
    }
}
