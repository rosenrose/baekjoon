use std::cmp::Ordering::{Equal, Greater, Less};
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());

    let a: Vec<_> = (0..10).map(|_| input.next().unwrap()).collect();
    let b = input;

    let (a_wins, b_wins) = a
        .iter()
        .zip(b)
        .map(|(a, b)| match a.cmp(&b) {
            Greater => (1, 0),
            Equal => (0, 0),
            Less => (0, 1),
        })
        .reduce(|(a1, b1), (a2, b2)| (a1 + a2, b1 + b2))
        .unwrap();

    match a_wins.cmp(&b_wins) {
        Greater => println!("A"),
        Equal => println!("D"),
        Less => println!("B"),
    };
}
