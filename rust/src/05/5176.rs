use std::collections::HashSet;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let mut input = || input.next().unwrap();

    for _ in 0..input() {
        let (p, _) = (input(), input());
        let seats: HashSet<_> = (0..p).map(|_| input()).collect();

        println!("{}", p - seats.len());
    }
}
