use std::io::{stdin, stdout, BufRead, BufWriter, Write};

fn main() {
    let (stdin, stdout) = (stdin(), stdout());
    let (mut stdin, mut stdout) = (stdin.lock(), BufWriter::new(stdout.lock()));

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();

    let n: usize = buf.trim().parse().unwrap();

    for _ in 0..n * 4 {
        writeln!(stdout, "{}", "@".repeat(n)).unwrap();
    }
    for _ in 0..n {
        writeln!(stdout, "{}", "@".repeat(n * 5)).unwrap();
    }
}
