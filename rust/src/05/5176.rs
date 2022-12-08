use std::collections::HashSet;
use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<usize>().unwrap());

    for _ in 0..input.next().unwrap() {
        let p = input.next().unwrap();
        input.next();

        let seats: HashSet<_> = (0..p).map(|_| input.next().unwrap()).collect();

        println!("{}", p - seats.len());
    }
}
