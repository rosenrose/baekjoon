use std::cmp::Ordering;
use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());
    let mut input = || input.next().unwrap();

    for _ in 0..input() {
        let (r, e, c) = (input(), input(), input());

        match r.cmp(&(e - c)) {
            Ordering::Less => println!("advertise"),
            Ordering::Equal => println!("does not matter"),
            Ordering::Greater => println!("do not advertise"),
        };
    }
}
