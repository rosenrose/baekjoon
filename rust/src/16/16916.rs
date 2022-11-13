use std::io::{stdin, stdout, BufWriter, Read, Write};

fn main() {
    let (stdin, stdout) = (stdin(), stdout());
    let (mut stdin, mut stdout) = (stdin.lock(), BufWriter::new(stdout.lock()));

    let mut buf = String::new();
    stdin.read_to_string(&mut buf).unwrap();

    let mut lines = buf.lines();
    let s = lines.next().unwrap();
    let p = lines.next().unwrap();

    writeln!(stdout, "{}", if s.contains(p) { 1 } else { 0 }).unwrap();
}
