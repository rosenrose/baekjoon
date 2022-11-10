use std::io::{stdin, stdout, BufWriter, Read, Write};

fn main() {
    let (stdin, stdout) = (stdin(), stdout());
    let (mut stdin, mut stdout) = (stdin.lock(), BufWriter::new(stdout.lock()));

    let mut buf = String::new();
    stdin.read_to_string(&mut buf).unwrap();

    let mut lines = buf.lines();
    lines.next();

    const N: i32 = 1_000_000;
    let (mut min, mut max) = (N, -N);

    lines
        .next()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .for_each(|num| {
            min = num.min(min);
            max = num.max(max);
        });

    writeln!(stdout, "{min} {max}").unwrap();
}
