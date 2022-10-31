use std::io::{stdin, stdout, BufRead, BufWriter, Write};

fn main() {
    let (stdin, stdout) = (stdin(), stdout());
    let (mut stdin, mut stdout) = (stdin.lock(), BufWriter::new(stdout.lock()));

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();

    let n: usize = buf.trim().parse().unwrap();
    let at = "@".repeat(n);
    let blank = n;

    for _ in 0..n {
        writeln!(stdout, "{}{:blank$}{at}", at.repeat(3), "").unwrap();
    }
    for _ in 0..n * 3 {
        writeln!(stdout, "{at}{:blank$}{at}{:blank$}{at}", "", "").unwrap();
    }
    for _ in 0..n {
        writeln!(stdout, "{at}{:blank$}{}", "", at.repeat(3),).unwrap();
    }
}
