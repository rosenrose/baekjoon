use std::cmp::Ordering;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());

    for _ in 0..input.next().unwrap() {
        let (y, k) = (0..9).fold((0, 0), |acc, _| {
            (acc.0 + input.next().unwrap(), acc.1 + input.next().unwrap())
        });

        match y.cmp(&k) {
            Ordering::Greater => println!("Yonsei"),
            Ordering::Less => println!("Korea"),
            Ordering::Equal => println!("Draw"),
        };
    }
}
