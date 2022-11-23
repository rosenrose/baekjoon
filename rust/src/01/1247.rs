use std::cmp::Ordering;
use std::io::{stdin, stdout, BufWriter, Read, Write};

fn main() {
    let (stdin, stdout) = (stdin(), stdout());
    let (mut stdin, mut stdout) = (stdin.lock(), BufWriter::new(stdout.lock()));

    let mut buf = String::new();
    stdin.read_to_string(&mut buf).unwrap();
    let mut lines = buf.lines();

    for _ in 0..3 {
        let n = parse_int(lines.next().unwrap());
        let s: i128 = (0..n).map(|_| parse_int(lines.next().unwrap())).sum();

        writeln!(
            stdout,
            "{}",
            match s.cmp(&0) {
                Ordering::Less => "-",
                Ordering::Equal => "0",
                Ordering::Greater => "+",
            }
        )
        .unwrap();
    }
}

fn parse_int(buf: &str) -> i128 {
    buf.parse().unwrap()
}
