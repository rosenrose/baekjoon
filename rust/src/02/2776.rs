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

    for _ in 0..input() {
        let n = input();
        let diary: HashSet<_> = (0..n).map(|_| input()).collect();

        let m = input();

        for num in (0..m).map(|_| input()) {
            writeln!(output, "{}", if diary.contains(&num) { 1 } else { 0 }).unwrap();
        }
    }

    print!("{output}");
}
