use std::cmp::Ordering;
use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());

    let (score_a, score_b) = (0..input.next().unwrap()).fold((100, 100), |acc, _| {
        let (a, b) = acc;
        let (a_num, b_num) = (input.next().unwrap(), input.next().unwrap());

        match a_num.cmp(&b_num) {
            Ordering::Less => (a - b_num, b),
            Ordering::Equal => (a, b),
            Ordering::Greater => (a, b - a_num),
        }
    });

    println!("{score_a}\n{score_b}");
}
