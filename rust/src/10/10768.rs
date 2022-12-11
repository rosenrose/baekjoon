use std::cmp::Ordering;
use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut input = buf.lines().map(|s| s.parse::<i32>().unwrap());
    let (month, date) = (input.next().unwrap(), input.next().unwrap());

    match (month, date).cmp(&(2, 18)) {
        Ordering::Less => println!("Before"),
        Ordering::Equal => println!("Special"),
        Ordering::Greater => println!("After"),
    }
}
