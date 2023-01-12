use std::cmp::Ordering::{Greater, Less};
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());
    let mut input = || input.next().unwrap();

    let (q1, q2, q3, q4, axis) = (0..input()).fold((0, 0, 0, 0, 0), |mut acc, _| {
        let (x, y) = (input(), input());

        match (x.cmp(&0), y.cmp(&0)) {
            (Greater, Greater) => acc.0 += 1,
            (Less, Greater) => acc.1 += 1,
            (Less, Less) => acc.2 += 1,
            (Greater, Less) => acc.3 += 1,
            _ => acc.4 += 1,
        };

        acc
    });

    println!("Q1: {q1}\nQ2: {q2}\nQ3: {q3}\nQ4: {q4}\nAXIS: {axis}");
}
