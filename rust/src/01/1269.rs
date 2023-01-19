use std::collections::HashSet;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut input = || input.next().unwrap();

    let (a, b) = (input(), input());
    let mut diff: HashSet<_> = (0..a).map(|_| input()).collect();

    for num in (0..b).map(|_| input()) {
        if diff.contains(&num) {
            diff.remove(&num);
        } else {
            diff.insert(num);
        }
    }

    println!("{}", diff.len());
}
