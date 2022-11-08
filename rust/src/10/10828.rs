use std::io::{stdin, stdout, BufRead, BufWriter, Write};

fn main() {
    let (stdin, stdout) = (stdin(), stdout());
    let (mut stdin, mut stdout) = (stdin.lock(), BufWriter::new(stdout.lock()));

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();

    let n = parse_int(buf.trim());
    let mut stack = Vec::new();

    for _ in 0..n {
        buf.clear();
        stdin.read_line(&mut buf).unwrap();

        let mut operation = buf.split_whitespace();

        match operation.next().unwrap() {
            "push" => stack.push(parse_int(operation.next().unwrap())),
            "pop" => writeln!(stdout, "{}", stack.pop().unwrap_or(-1)).unwrap(),
            "size" => writeln!(stdout, "{}", stack.len()).unwrap(),
            "empty" => writeln!(stdout, "{}", if stack.is_empty() { 1 } else { 0 }).unwrap(),
            "top" => writeln!(stdout, "{}", stack.last().unwrap_or(&-1)).unwrap(),
            _ => (),
        };
    }
}

fn parse_int(buf: &str) -> i32 {
    buf.parse().unwrap()
}
