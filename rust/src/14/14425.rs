use std::collections::HashSet;
use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut input = buf.split_ascii_whitespace();

    let n: i32 = input.next().unwrap().parse().unwrap();
    input.next();

    let word_set: HashSet<_> = (0..n).map(|_| input.next().unwrap()).collect();
    let count = input.filter(|word| word_set.contains(word)).count();

    println!("{count}");
}
