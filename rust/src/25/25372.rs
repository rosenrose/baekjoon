use std::fmt::Write;
use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let input = buf.split_ascii_whitespace();
    let mut output = String::new();

    for password in input.skip(1) {
        writeln!(
            output,
            "{}",
            match password.len() {
                6..=9 => "yes",
                _ => "no",
            }
        )
        .unwrap();
    }

    print!("{output}");
}
