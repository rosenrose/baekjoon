use std::io::{stdin, stdout, BufRead, BufWriter, Write};

fn main() {
    let (stdin, stdout) = (stdin(), stdout());
    let (mut stdin, mut stdout) = (stdin.lock(), BufWriter::new(stdout.lock()));

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();

    let parse_int = |s: &str| s.parse::<i32>().unwrap();
    let n: i32 = parse_int(buf.trim());

    for i in 1..=n {
        buf.clear();
        stdin.read_line(&mut buf).unwrap();

        let sum: i32 = buf.split_whitespace().map(parse_int).sum();

        writeln!(stdout, "Case {i}: {sum}").unwrap();
    }
}
