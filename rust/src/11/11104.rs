use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut output = String::new();

    for input in buf.lines().skip(1) {
        writeln!(output, "{}", i32::from_str_radix(input, 2).unwrap()).unwrap();
    }

    print!("{output}");
}
