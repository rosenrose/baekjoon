use std::fmt::Write;
use std::io::{self, Read};

fn main() {
    let mut buf = Vec::new();
    io::stdin().read_to_end(&mut buf).unwrap();

    let mut input = buf.split(|&ch| matches!(ch, b' ' | b'\n')).map(parse_int);
    let mut input = || input.next().unwrap();
    let mut output = String::new();

    for (a, b) in (0..input()).map(|_| (input(), input())) {
        writeln!(output, "{}", a + b).unwrap();
    }

    print!("{output}");
}

fn parse_int(input: &[u8]) -> i32 {
    input
        .iter()
        .fold(0, |acc, &ch| acc * 10 + (ch - b'0') as i32)
}
