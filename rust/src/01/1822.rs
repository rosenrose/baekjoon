use std::collections::HashSet;
use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());
    let mut input = || input.next().unwrap();
    let mut output = String::new();

    let (na, nb) = (input(), input());
    let mut a: HashSet<_> = (0..na).map(|_| input()).collect();

    for num in (0..nb).map(|_| input()) {
        a.remove(&num);
    }

    let mut a = Vec::from_iter(a);
    a.sort_unstable();

    println!("{}", a.len());

    for num in a {
        write!(output, "{num} ").unwrap();
    }

    print!("{output}");
}
