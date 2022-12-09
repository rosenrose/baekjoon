use std::collections::HashSet;
use std::fmt::Write;
use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());
    let mut output = String::new();

    for _ in 0..input.next().unwrap() {
        let n = input.next().unwrap();
        let diary: HashSet<_> = (0..n).map(|_| input.next().unwrap()).collect();

        let m = input.next().unwrap();

        for num in (0..m).map(|_| input.next().unwrap()) {
            writeln!(output, "{}", if diary.contains(&num) { 1 } else { 0 }).unwrap();
        }
    }

    print!("{output}");
}
