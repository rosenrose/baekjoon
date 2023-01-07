use std::cmp::Ordering;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
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
