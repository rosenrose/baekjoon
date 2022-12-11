use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut input = buf.lines().map(|s| s.parse::<i32>().unwrap());

    let (n, mut p) = (input.next().unwrap(), input.next().unwrap());
    let discounts = [
        0,
        500,
        (p as f64 * 0.1) as i32,
        2000,
        (p as f64 * 0.25) as i32,
    ];

    p -= discounts[..=(n / 5).min(4) as usize]
        .into_iter()
        .max()
        .unwrap();

    println!("{}", p.max(0));
}
