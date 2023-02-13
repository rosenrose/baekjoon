use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut output = String::new();

    for input in buf.lines() {
        writeln!(
            output,
            "{}",
            if input.to_lowercase().contains("problem") {
                "yes"
            } else {
                "no"
            }
        )
        .unwrap();
    }

    print!("{output}");
}
