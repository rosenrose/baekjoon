use std::cmp::Ordering::{Greater, Less};
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut input = || input.next().unwrap();

    let mut counts = [0; 5];

    for (x, y) in (0..input()).map(|_| (input(), input())) {
        let idx = match (x.cmp(&0), y.cmp(&0)) {
            (Greater, Greater) => 0,
            (Less, Greater) => 1,
            (Less, Less) => 2,
            (Greater, Less) => 3,
            _ => 4,
        };

        counts[idx] += 1;
    }

    let [q1, q2, q3, q4, axis] = counts;

    println!("Q1: {q1}\nQ2: {q2}\nQ3: {q3}\nQ4: {q4}\nAXIS: {axis}");
}
