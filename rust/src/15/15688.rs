use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut output = String::new();

    let mut arr: Vec<_> = input.skip(1).collect();
    arr.sort_unstable();

    for num in arr {
        writeln!(output, "{num}").unwrap();
    }

    print!("{output}");
}
