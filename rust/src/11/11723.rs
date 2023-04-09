use std::io::{self, BufRead, Write};

fn main() {
    let mut stdin = io::stdin().lock();
    let mut stdout = io::BufWriter::new(io::stdout().lock());

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();

    let mut s = 0;

    for _ in 0..parse_int(buf.trim_end()) {
        buf.clear();
        stdin.read_line(&mut buf).unwrap();

        let mut input = buf.split_ascii_whitespace();
        let op = input.next().unwrap();
        let x = if let Some(s) = input.next() {
            parse_int(s)
        } else {
            0
        };

        match op {
            "add" => s |= 1 << x,
            "remove" => s &= !(1 << x),
            "check" => {
                let check = 1 << x;
                writeln!(stdout, "{}", if s & check == check { 1 } else { 0 }).unwrap();
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
