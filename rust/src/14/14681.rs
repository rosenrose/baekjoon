use std::cmp::Ordering::{Greater, Less};
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.lines().flat_map(str::parse::<i32>);
    let (x, y) = (input.next().unwrap(), input.next().unwrap());

    match (x.cmp(&0), y.cmp(&0)) {
        (Greater, Greater) => println!("1"),
        (Less, Greater) => println!("2"),
        (Less, Less) => println!("3"),
        (Greater, Less) => println!("4"),
        _ => (),
    }
}
