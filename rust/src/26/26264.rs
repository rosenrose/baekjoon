use std::cmp::Ordering;
use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let input = buf.lines().next_back().unwrap();

    match input
        .matches("bigdata")
        .count()
        .cmp(&input.matches("security").count())
    {
        Ordering::Less => println!("security!"),
        Ordering::Equal => println!("bigdata? security!"),
        Ordering::Greater => println!("bigdata?"),
    }
}
