use std::collections::HashSet;
use std::io::{stdin, stdout, BufRead, BufWriter, Write};

fn main() {
    let stdin = stdin();
    let stdout = stdout();
    let mut stdin = stdin.lock();
    let mut stdout = BufWriter::new(stdout.lock());

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    buf.clear();
    stdin.read_line(&mut buf).unwrap();

    let card_set: HashSet<i32> = buf.split_whitespace().map(|s| s.parse().unwrap()).collect();
    stdin.read_line(&mut buf).unwrap();
    buf.clear();
    stdin.read_line(&mut buf).unwrap();

    let nums = buf.split_whitespace().map(|s| s.parse::<i32>().unwrap());

    for num in nums {
        match card_set.contains(&num) {
            true => write!(stdout, "1 ").unwrap(),
            false => write!(stdout, "0 ").unwrap(),
        }
    }
}
