use std::cmp::Ordering;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());
    let mut input = || input.next().unwrap();

    let (a_wins, b_wins) = (0..input())
        .map(|_| {
            let (a, b) = (input(), input());

            match a.cmp(&b) {
                Ordering::Greater => (1, 0),
                Ordering::Equal => (0, 0),
                Ordering::Less => (0, 1),
            }
        })
        .reduce(|(a1, b1), (a2, b2)| (a1 + a2, b1 + b2))
        .unwrap();

    println!("{a_wins} {b_wins}");
}
