use std::collections::HashSet;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);

    for _ in 0..input.next().unwrap() {
        let [p, _] = [(); 2].map(|_| input.next().unwrap());
        let seats: HashSet<_> = input.by_ref().take(p).collect();

        println!("{}", p - seats.len());
    }
}
