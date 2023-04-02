use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut output = String::new();

    for input in buf.lines().skip(1) {
        writeln!(
            output,
            "{}",
            if matches!(input.len(), 6..=9) {
                "yes"
            } else {
                "no"
            }
        )
        .unwrap();
    }

    print!("{output}");
}
