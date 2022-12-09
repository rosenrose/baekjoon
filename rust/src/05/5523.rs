use std::cmp::Ordering;
use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());

    let (a_wins, b_wins) = (0..input.next().unwrap())
        .map(|_| {
            let (a, b) = (input.next().unwrap(), input.next().unwrap());

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
