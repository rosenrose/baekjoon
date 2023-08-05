use std::collections::HashSet;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();

    let [n, _] = [(); 2].map(|_| input.next().unwrap().parse::<usize>().unwrap());
    let not_heard: HashSet<_> = input.by_ref().take(n).collect();

    let mut not_heard_seen = Vec::from_iter(input.filter(|not_seen| not_heard.contains(not_seen)));
    not_heard_seen.sort();

    println!("{}\n{}", not_heard_seen.len(), not_heard_seen.join("\n"));
}
