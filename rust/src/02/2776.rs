use std::collections::HashSet;
use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut input = || input.next().unwrap();
    let mut output = String::new();

    for _ in 0..input() {
        let diary: HashSet<_> = (0..input()).map(|_| input()).collect();

        for num in (0..input()).map(|_| input()) {
            writeln!(output, "{}", if diary.contains(&num) { 1 } else { 0 }).unwrap();
        }
    }

    print!("{output}");
}
