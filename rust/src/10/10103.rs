use std::cmp::Ordering;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());
    let mut input = || input.next().unwrap();

    let (score_a, score_b) = (0..input()).fold((100, 100), |(a, b), _| {
        let (a_num, b_num) = (input(), input());

        match a_num.cmp(&b_num) {
            Ordering::Less => (a - b_num, b),
            Ordering::Equal => (a, b),
            Ordering::Greater => (a, b - a_num),
        }
    });

    println!("{score_a}\n{score_b}");
}
