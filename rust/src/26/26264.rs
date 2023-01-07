use std::cmp::Ordering;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
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
