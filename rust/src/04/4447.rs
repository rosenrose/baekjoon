use std::cmp::Ordering;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let input = buf.lines();

    for hero in input.skip(1) {
        let lower = hero.to_lowercase();

        match lower.matches('g').count().cmp(&lower.matches('b').count()) {
            Ordering::Greater => println!("{hero} is GOOD"),
            Ordering::Equal => println!("{hero} is NEUTRAL"),
            Ordering::Less => println!("{hero} is A BADDY"),
        }
    }
}
