use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut output = String::new();

    const SIMON: &str = "Simon says";

    for input in buf.lines().skip(1) {
        if input.starts_with(SIMON) {
            writeln!(output, "{}", &input[SIMON.len()..]).unwrap();
        }
    }

    print!("{output}");
}
