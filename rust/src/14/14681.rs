use std::cmp::Ordering::{Greater, Less};
use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut input = buf.lines().map(|s| s.parse::<i32>().unwrap());
    let (x, y) = (input.next().unwrap(), input.next().unwrap());

    match (x.cmp(&0), y.cmp(&0)) {
        (Greater, Greater) => println!("1"),
        (Less, Greater) => println!("2"),
        (Less, Less) => println!("3"),
        (Greater, Less) => println!("4"),
        _ => (),
    }
}
