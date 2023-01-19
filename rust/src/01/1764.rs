use std::collections::HashSet;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();

    let n: i32 = input.next().unwrap().parse().unwrap();
    let _m = input.next();
    let not_heard: HashSet<_> = (0..n).map(|_| input.next().unwrap()).collect();

    let mut not_heard_seen: Vec<_> = input
        .filter(|not_seen| not_heard.contains(not_seen))
        .collect();
    not_heard_seen.sort();

    println!("{}\n{}", not_heard_seen.len(), not_heard_seen.join("\n"));
}
