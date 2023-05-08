use std::io::{self, BufRead, Write};

fn main() {
    let mut stdin = io::stdin().lock();
    let mut stdout = io::BufWriter::new(io::stdout().lock());

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();

    let mut s = 0;
    let mut bits = [0; 21];

    for i in 1..=20 {
        bits[i] = 1 << i;
    }

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
            "add" => s |= bits[x],
            "remove" => s &= !bits[x],
            "check" => writeln!(stdout, "{}", u8::from(s & bits[x] == bits[x])).unwrap(),
            "toggle" => s ^= bits[x],
            "all" => s |= !0,
            "empty" => s = 0,
            _ => (),
        };
    }
}

fn parse_int(buf: &str) -> usize {
    buf.parse().unwrap()
}
