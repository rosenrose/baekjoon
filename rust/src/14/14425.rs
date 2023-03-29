use std::collections::HashSet;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();

    let (n, _) = (
        input.next().unwrap().parse::<usize>().unwrap(),
        input.next(),
    );
    let word_set: HashSet<_> = input.by_ref().take(n).collect();
    let count = input.filter(|word| word_set.contains(word)).count();

    println!("{count}");
}
