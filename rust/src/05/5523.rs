use std::cmp::Ordering;
use std::io::{stdin, stdout, BufWriter, Read, Write};

fn main() {
    let (stdin, stdout) = (stdin(), stdout());
    let (mut stdin, mut stdout) = (stdin.lock(), BufWriter::new(stdout.lock()));

    let mut buf = String::new();
    stdin.read_to_string(&mut buf).unwrap();

    let (a, b) = buf
        .lines()
        .skip(1)
        .map(|line| {
            let mut tokens = line.split_whitespace();

            let a = parse_int(tokens.next().unwrap());
            let b = parse_int(tokens.next().unwrap());

            match a.cmp(&b) {
                Ordering::Greater => (1, 0),
                Ordering::Equal => (0, 0),
                Ordering::Less => (0, 1),
            }
        })
        .reduce(|(a1, b1), (a2, b2)| (a1 + a2, b1 + b2))
        .unwrap();

    writeln!(stdout, "{a} {b}").unwrap();
}

fn parse_int(buf: &str) -> i32 {
    buf.parse().unwrap()
}
