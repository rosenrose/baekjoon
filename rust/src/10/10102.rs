use std::cmp::Ordering;
use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let votes = buf.lines().next_back().unwrap();
    let a_count = votes.chars().filter(|&c| c == 'A').count();

    match (votes.len() - a_count).cmp(&a_count) {
        Ordering::Less => println!("A"),
        Ordering::Equal => println!("Tie"),
        Ordering::Greater => println!("B"),
    };
}
