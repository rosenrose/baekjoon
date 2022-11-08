use std::collections::VecDeque;
use std::io::{stdin, stdout, BufRead, BufWriter, Write};

fn main() {
    let (stdin, stdout) = (stdin(), stdout());
    let (mut stdin, mut stdout) = (stdin.lock(), BufWriter::new(stdout.lock()));

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();

    let n = parse_int(buf.trim());
    let mut queue = VecDeque::new();

    for _ in 0..n {
        buf.clear();
        stdin.read_line(&mut buf).unwrap();

        let mut operation = buf.split_whitespace();

        match operation.next().unwrap() {
            "push" => queue.push_back(parse_int(operation.next().unwrap())),
            "pop" => writeln!(stdout, "{}", queue.pop_front().unwrap_or(-1)).unwrap(),
            "size" => writeln!(stdout, "{}", queue.len()).unwrap(),
            "empty" => writeln!(stdout, "{}", if queue.is_empty() { 1 } else { 0 }).unwrap(),
            "front" => writeln!(stdout, "{}", queue.front().unwrap_or(&-1)).unwrap(),
            "back" => writeln!(stdout, "{}", queue.back().unwrap_or(&-1)).unwrap(),
            _ => (),
        };
    }
}

fn parse_int(buf: &str) -> i32 {
    buf.parse().unwrap()
}
