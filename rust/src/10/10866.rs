use std::collections::VecDeque;
use std::io::{stdin, stdout, BufRead, BufWriter, Write};

fn main() {
    let (stdin, stdout) = (stdin(), stdout());
    let (mut stdin, mut stdout) = (stdin.lock(), BufWriter::new(stdout.lock()));

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();

    let n = parse_int(buf.trim());
    let mut deque = VecDeque::new();

    for _ in 0..n {
        buf.clear();
        stdin.read_line(&mut buf).unwrap();

        let mut operation = buf.split_whitespace();

        match operation.next().unwrap() {
            "push_front" => deque.push_front(parse_int(operation.next().unwrap())),
            "push_back" => deque.push_back(parse_int(operation.next().unwrap())),
            "pop_front" => writeln!(stdout, "{}", deque.pop_front().unwrap_or(-1)).unwrap(),
            "pop_back" => writeln!(stdout, "{}", deque.pop_back().unwrap_or(-1)).unwrap(),
            "size" => writeln!(stdout, "{}", deque.len()).unwrap(),
            "empty" => writeln!(stdout, "{}", if deque.is_empty() { 1 } else { 0 }).unwrap(),
            "front" => writeln!(stdout, "{}", deque.front().unwrap_or(&-1)).unwrap(),
            "back" => writeln!(stdout, "{}", deque.back().unwrap_or(&-1)).unwrap(),
            _ => (),
        };
    }
}

fn parse_int(buf: &str) -> i32 {
    buf.parse().unwrap()
}
