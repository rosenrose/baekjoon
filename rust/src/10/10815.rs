use std::collections::HashSet;
use std::io::{stdin, stdout, BufRead, BufWriter, Write};

fn main() {
    let (stdin, stdout) = (stdin(), stdout());
    let (mut stdin, mut stdout) = (stdin.lock(), BufWriter::new(stdout.lock()));

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    buf.clear();
    stdin.read_line(&mut buf).unwrap();

    let parse_int = |s: &str| s.parse::<i32>().unwrap();
    let card_set: HashSet<i32> = buf.split_whitespace().map(parse_int).collect();

    stdin.read_line(&mut buf).unwrap();
    buf.clear();
    stdin.read_line(&mut buf).unwrap();

    for num in buf.split_whitespace().map(parse_int) {
        let has_card = match card_set.contains(&num) {
            true => 1,
            false => 0,
        };

        write!(stdout, "{has_card} ").unwrap();
    }
}
