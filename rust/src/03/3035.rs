use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();
    let mut input = || input.next().unwrap();
    let mut output = String::new();

    let (r, _, zr, zc) = (
        parse_int(input()),
        input(),
        parse_int(input()),
        parse_int(input()),
    );

    for row in (0..r).map(|_| input()) {
        for _ in 0..zr {
            for ch in row.chars() {
                write!(output, "{}", ch.to_string().repeat(zc)).unwrap();
            }
            writeln!(output, "").unwrap();
        }
    }

    print!("{output}");
}

fn parse_int(buf: &str) -> usize {
    buf.parse().unwrap()
}
