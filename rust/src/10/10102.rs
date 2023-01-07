use std::cmp::Ordering;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let votes = buf.lines().next_back().unwrap();
    let a_count = votes.chars().filter(|&c| c == 'A').count();

    match (votes.len() - a_count).cmp(&a_count) {
        Ordering::Less => println!("A"),
        Ordering::Equal => println!("Tie"),
        Ordering::Greater => println!("B"),
    };
}
