use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut output = String::new();

    for num in input.skip(1) {
        writeln!(output, "{}", u8::from(num & -num == num)).unwrap();
    }

    print!("{output}");
}
