use std::collections::HashSet;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);

    for _ in 0..input.next().unwrap() {
        let (p, _) = (input.next().unwrap(), input.next());
        let seats: HashSet<_> = input.by_ref().take(p).collect();

        println!("{}", p - seats.len());
    }
}
