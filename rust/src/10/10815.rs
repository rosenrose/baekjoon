use std::collections::HashSet;
use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut output = String::new();

    let card_set: HashSet<_> = (0..input.next().unwrap())
        .map(|_| input.next().unwrap())
        .collect();

    for num in input.skip(1) {
        write!(output, "{} ", if card_set.contains(&num) { 1 } else { 0 }).unwrap();
    }

    print!("{output}");
}
