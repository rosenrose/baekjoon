use std::cmp::Ordering;
use std::io::{stdin, Read};

fn main() {
    let stdin = stdin();
    let mut stdin = stdin.lock();

    let mut buf = String::new();
    stdin.read_to_string(&mut buf).unwrap();

    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i128>().unwrap());

    for _ in 0..3 {
        let n = input.next().unwrap();
        let sum: i128 = (0..n).map(|_| input.next().unwrap()).sum();

        println!(
            "{}",
            match sum.cmp(&0) {
                Ordering::Less => "-",
                Ordering::Equal => "0",
                Ordering::Greater => "+",
            }
        );
    }
}
