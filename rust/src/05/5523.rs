use std::cmp::Ordering;
use std::io::{stdin, stdout, BufRead, BufWriter, Write};

fn main() {
    let (stdin, stdout) = (stdin(), stdout());
    let (mut stdin, mut stdout) = (stdin.lock(), BufWriter::new(stdout.lock()));

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();

    let n = parse_int(buf.trim());

    let (a, b) = (0..n)
        .map(|_| {
            buf.clear();
            stdin.read_line(&mut buf).unwrap();
            let mut tokens = buf.split_whitespace();

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
