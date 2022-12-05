use std::cmp::Ordering;
use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());

    let good_point = [1, 2, 3, 3, 4, 10];
    let evil_point = [1, 2, 2, 2, 3, 5, 10];

    for i in 1..=input.next().unwrap() {
        let good: i32 = good_point
            .iter()
            .map(|point| point * input.next().unwrap())
            .sum();
        let evil: i32 = evil_point
            .iter()
            .map(|point| point * input.next().unwrap())
            .sum();

        match good.cmp(&evil) {
            Ordering::Less => println!("Battle {i}: Evil eradicates all trace of Good"),
            Ordering::Equal => println!("Battle {i}: No victor on this battle field"),
            Ordering::Greater => println!("Battle {i}: Good triumphs over Evil"),
        }
    }
}
