use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut output = String::new();

    for input in buf.lines().skip(1) {
        if input.contains('=') {
            writeln!(output, "skipped").unwrap();
            continue;
        }

        let sum: i32 = input.split('+').flat_map(str::parse::<i32>).sum();

        writeln!(output, "{sum}").unwrap();
    }

    print!("{output}");
}
