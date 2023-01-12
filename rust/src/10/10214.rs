use std::cmp::Ordering;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());
    let mut input = || input.next().unwrap();

    for _ in 0..input() {
        let (y, k) = (0..9).fold((0, 0), |(y, k), _| (y + input(), k + input()));

        match y.cmp(&k) {
            Ordering::Greater => println!("Yonsei"),
            Ordering::Less => println!("Korea"),
            Ordering::Equal => println!("Draw"),
        };
    }
}
