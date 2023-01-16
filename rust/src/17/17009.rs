use std::cmp::Ordering;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.lines().flat_map(str::parse::<i32>);

    let apple: i32 = (1..=3).rev().map(|i| input.next().unwrap() * i).sum();
    let banana: i32 = (1..=3).rev().map(|i| input.next().unwrap() * i).sum();

    match apple.cmp(&banana) {
        Ordering::Greater => println!("A"),
        Ordering::Less => println!("B"),
        Ordering::Equal => println!("T"),
    }
}
