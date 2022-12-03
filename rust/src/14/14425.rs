use std::collections::HashSet;
use std::io::{stdin, Read};

fn main() {
    let stdin = stdin();
    let mut stdin = stdin.lock();

    let mut buf = String::new();
    stdin.read_to_string(&mut buf).unwrap();

    let mut input = buf.split_ascii_whitespace();

    let n = parse_int(input.next().unwrap());
    input.next();

    let word_set: HashSet<_> = (0..n).map(|_| input.next().unwrap()).collect();
    let count = input.filter(|word| word_set.contains(word)).count();

    println!("{count}");
}

fn parse_int(buf: &str) -> i32 {
    buf.parse().unwrap()
}
