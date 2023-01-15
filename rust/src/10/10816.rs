use std::collections::HashMap;
use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut output = String::new();

    let n = input.next().unwrap();
    let counts = (0..n).fold(HashMap::new(), |mut acc, _| {
        acc.entry(input.next().unwrap())
            .and_modify(|c| *c += 1)
            .or_insert(1);

        acc
    });

    for num in input.skip(1) {
        write!(output, "{} ", counts.get(&num).unwrap_or(&0)).unwrap();
    }

    print!("{output}");
}
