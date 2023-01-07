use std::cmp::Ordering;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.lines().map(|s| s.parse::<i32>().unwrap());
    let (month, date) = (input.next().unwrap(), input.next().unwrap());

    match (month, date).cmp(&(2, 18)) {
        Ordering::Less => println!("Before"),
        Ordering::Equal => println!("Special"),
        Ordering::Greater => println!("After"),
    }
}
