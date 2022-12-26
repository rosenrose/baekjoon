use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<f64>().unwrap());

    let prices = [350.34, 230.90, 190.55, 125.30, 180.90];

    for _ in 0..input.next().unwrap() as i32 {
        let cost: f64 = prices
            .iter()
            .map(|price| price * input.next().unwrap())
            .sum();

        println!("${cost:.2}");
    }
}
