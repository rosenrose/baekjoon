use std::io::{self, BufRead, Write};

fn main() {
    let mut stdin = io::stdin().lock();
    let mut stdout = io::BufWriter::new(io::stdout().lock());

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();

    let n: i32 = buf.trim_end().parse().unwrap();
    const MAX: usize = 10_000;

    let mut counts = [0; MAX + 1];

    for _ in 0..n {
        buf.clear();
        stdin.read_line(&mut buf).unwrap();

        let num: usize = buf.trim_end().parse().unwrap();
        counts[num] += 1;
    }

    for num in 0..=MAX {
        for _ in 0..counts[num] {
            writeln!(stdout, "{num}").unwrap();
        }
    }
}
