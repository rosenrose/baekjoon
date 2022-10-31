use std::io::{stdin, stdout, BufRead, BufWriter, Write};

fn main() {
    let (stdin, stdout) = (stdin(), stdout());
    let (mut stdin, mut stdout) = (stdin.lock(), BufWriter::new(stdout.lock()));

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();

    let n: usize = buf.trim().parse().unwrap();
    let at = "@".repeat(n);
    let long_at = at.repeat(5);

    for _ in 0..n {
        writeln!(stdout, "{long_at}").unwrap();
    }
    for _ in 0..n {
        writeln!(stdout, "{at}").unwrap();
    }
    for _ in 0..n {
        writeln!(stdout, "{long_at}").unwrap();
    }
    for _ in 0..n {
        writeln!(stdout, "{at}").unwrap();
    }
    for _ in 0..n {
        writeln!(stdout, "{long_at}").unwrap();
    }
}
