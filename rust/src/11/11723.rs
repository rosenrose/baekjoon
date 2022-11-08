use std::io::{stdin, stdout, BufRead, BufWriter, Write};

fn main() {
    let (stdin, stdout) = (stdin(), stdout());
    let (mut stdin, mut stdout) = (stdin.lock(), BufWriter::new(stdout.lock()));

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();

    let n = parse_int(buf.trim());
    let mut s = 0;

    for _ in 0..n {
        buf.clear();
        stdin.read_line(&mut buf).unwrap();

        let mut operation = buf.split_whitespace();
        let op = operation.next().unwrap();
        let x = parse_int(operation.next().unwrap_or("0"));

        match op {
            "add" => s |= 1 << x,
            "remove" => s &= !(1 << x),
            "check" => {
                writeln!(stdout, "{}", if (s & (1 << x)) >> x == 1 { 1 } else { 0 }).unwrap();
            }
            "toggle" => s ^= 1 << x,
            "all" => s |= !0,
            "empty" => s = 0,
            _ => (),
        };
    }
}

fn parse_int(buf: &str) -> i32 {
    buf.parse().unwrap()
}
