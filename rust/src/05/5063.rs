use std::cmp::Ordering;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut input = || input.next().unwrap();

    for (r, e, c) in (0..input()).map(|_| (input(), input(), input())) {
        match r.cmp(&(e - c)) {
            Ordering::Less => println!("advertise"),
            Ordering::Equal => println!("does not matter"),
            Ordering::Greater => println!("do not advertise"),
        };
    }
}
