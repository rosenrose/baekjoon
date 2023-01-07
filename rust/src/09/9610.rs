use std::cmp::Ordering::{Greater, Less};
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());

    let (mut q1, mut q2, mut q3, mut q4, mut axis) = (0, 0, 0, 0, 0);

    for _ in 0..input.next().unwrap() {
        let (x, y) = (input.next().unwrap(), input.next().unwrap());

        match (x.cmp(&0), y.cmp(&0)) {
            (Greater, Greater) => q1 += 1,
            (Less, Greater) => q2 += 1,
            (Less, Less) => q3 += 1,
            (Greater, Less) => q4 += 1,
            _ => axis += 1,
        };
    }

    println!("Q1: {q1}\nQ2: {q2}\nQ3: {q3}\nQ4: {q4}\nAXIS: {axis}");
}
