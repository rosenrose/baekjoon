use std::io::{stdin, stdout, BufWriter, Read, Write};

fn main() {
    let (stdin, stdout) = (stdin(), stdout());
    let (mut stdin, mut stdout) = (stdin.lock(), BufWriter::new(stdout.lock()));

    let mut buf = String::new();
    stdin.read_to_string(&mut buf).unwrap();

    buf.lines().skip(1).for_each(|line| {
        let sum: i32 = line
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .sum();

        writeln!(stdout, "{sum}").unwrap();
    });
}
