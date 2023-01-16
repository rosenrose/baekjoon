use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut output = String::new();

    for n in buf.lines().skip(1).flat_map(str::parse::<i64>) {
        writeln!(output, "{}", n * n).unwrap();
    }

    print!("{output}");
}
