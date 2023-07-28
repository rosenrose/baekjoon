use std::collections::HashSet;
use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut output = String::new();

    let n = input.next().unwrap() as usize;
    let nums: HashSet<_> = input.by_ref().take(n).collect();

    for num in input.skip(1) {
        writeln!(output, "{}", nums.contains(&num) as u8).unwrap();
    }

    print!("{output}");
}
