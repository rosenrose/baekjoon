use std::cmp::Ordering;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut input = || input.next().unwrap();

    let good_points = [1, 2, 3, 3, 4, 10];
    let evil_points = [1, 2, 2, 2, 3, 5, 10];

    for i in 1..=input() {
        let good: i32 = good_points.iter().map(|point| point * input()).sum();
        let evil: i32 = evil_points.iter().map(|point| point * input()).sum();

        match good.cmp(&evil) {
            Ordering::Less => println!("Battle {i}: Evil eradicates all trace of Good"),
            Ordering::Equal => println!("Battle {i}: No victor on this battle field"),
            Ordering::Greater => println!("Battle {i}: Good triumphs over Evil"),
        }
    }
}
