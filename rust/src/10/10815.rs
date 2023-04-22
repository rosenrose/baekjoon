use std::collections::HashSet;
use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut output = String::new();

    let n = input.next().unwrap() as usize;
    let mut cards = HashSet::with_capacity(n);

    for num in input.by_ref().take(n) {
        cards.insert(num);
    }

    for num in input.skip(1) {
        write!(output, "{} ", if cards.contains(&num) { 1 } else { 0 }).unwrap();
    }

    print!("{output}");
}
