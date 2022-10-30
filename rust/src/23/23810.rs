use std::io::{stdin, stdout, BufRead, BufWriter, Write};

fn main() {
    let stdin = stdin();
    let stdout = stdout();
    let mut stdin = stdin.lock();
    let mut stdout = BufWriter::new(stdout.lock());

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();

    let n: usize = buf.trim().parse().unwrap();
    let at = "@".repeat(n);

    for _ in 0..n {
        writeln!(stdout, "{}", at.repeat(5)).unwrap();
    }
    for _ in 0..n {
        writeln!(stdout, "{at}").unwrap();
    }
    for _ in 0..n {
        writeln!(stdout, "{}", at.repeat(5)).unwrap();
    }
    for _ in 0..n * 2 {
        writeln!(stdout, "{at}").unwrap();
    }
}
