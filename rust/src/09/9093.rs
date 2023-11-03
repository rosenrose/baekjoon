use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut output = String::with_capacity(50_000);

    for input in buf.lines().skip(1) {
        for reversed in input.split(' ').map(|word| word.chars().rev()) {
            for ch in reversed {
                output.push(ch);
            }
            output.push(' ');
        }

        writeln!(output, "").unwrap();
    }

    print!("{output}");
}
