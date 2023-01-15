use std::cmp::Ordering;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let s = buf.lines().next_back().unwrap();

    match s.matches('2').count().cmp(&s.matches('e').count()) {
        Ordering::Greater => println!("2"),
        Ordering::Equal => println!("yee"),
        Ordering::Less => println!("e"),
    }
}
