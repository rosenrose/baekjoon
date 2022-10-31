use std::collections::HashMap;
use std::io::{stdin, stdout, BufRead, BufWriter, Write};

fn main() {
    let (stdin, stdout) = (stdin(), stdout());
    let (mut stdin, mut stdout) = (stdin.lock(), BufWriter::new(stdout.lock()));

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    buf.clear();
    stdin.read_line(&mut buf).unwrap();

    let mut counts = HashMap::new();
    let parse_int = |s: &str| s.parse::<i32>().unwrap();

    for num in buf.split_whitespace().map(parse_int) {
        counts.entry(num).and_modify(|c| *c += 1).or_insert(1);
    }

    stdin.read_line(&mut buf).unwrap();
    buf.clear();
    stdin.read_line(&mut buf).unwrap();

    for num in buf.split_whitespace().map(parse_int) {
        let count = match counts.get(&num) {
            Some(c) => *c,
            None => 0,
        };

        write!(stdout, "{count} ").unwrap();
    }
}
