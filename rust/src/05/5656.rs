use std::io::{stdin, stdout, BufWriter, Read, Write};

fn main() {
    let (stdin, stdout) = (stdin(), stdout());
    let (mut stdin, mut stdout) = (stdin.lock(), BufWriter::new(stdout.lock()));

    let mut buf = String::new();
    stdin.read_to_string(&mut buf).unwrap();

    let parse_int = |s: &str| s.parse::<i32>().unwrap();

    for (i, line) in buf.lines().enumerate() {
        let mut tokens = line.split_whitespace();

        let a = parse_int(tokens.next().unwrap());
        let op = tokens.next().unwrap();
        let b = parse_int(tokens.next().unwrap());

        let cmp = match op {
            ">" => a > b,
            ">=" => a >= b,
            "<" => a < b,
            "<=" => a <= b,
            "==" => a == b,
            "!=" => a != b,
            _ => return,
        };

        writeln!(stdout, "Case {}: {cmp}", i + 1).unwrap();
    }
}
