use std::collections::HashMap;
use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut output = String::new();

    let n = input.next().unwrap() as usize;
    let mut counts = HashMap::with_capacity(n >> 1);

    for num in input.by_ref().take(n) {
        counts.entry(num).and_modify(|c| *c += 1).or_insert(1);
    }

    for num in input.skip(1) {
        write!(output, "{} ", counts.get(&num).unwrap_or(&0)).unwrap();
    }

    print!("{output}");
}
