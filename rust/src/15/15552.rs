use std::io::{stdin, stdout, BufRead, BufWriter, Write};

fn main() {
    let (stdin, stdout) = (stdin(), stdout());
    let (mut stdin, mut stdout) = (stdin.lock(), BufWriter::new(stdout.lock()));

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();

    let n: i32 = buf.trim().parse().unwrap();

    for _ in 0..n {
        buf.clear();
        stdin.read_line(&mut buf).unwrap();

        let sum: i32 = buf
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .sum();

        writeln!(stdout, "{sum}").unwrap();
    }
}
