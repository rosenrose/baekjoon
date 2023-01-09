use std::cmp::Ordering;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());
    let mut input = || input.next().unwrap();

    let (a_atk, a_hp, b_atk, b_hp) = (input(), input(), input(), input());
    let a_turns = (b_hp as f64 / a_atk as f64).ceil() as i32;
    let b_turns = (a_hp as f64 / b_atk as f64).ceil() as i32;

    match a_turns.cmp(&b_turns) {
        Ordering::Less => println!("PLAYER A"),
        Ordering::Equal => println!("DRAW"),
        Ordering::Greater => println!("PLAYER B"),
    }
}
