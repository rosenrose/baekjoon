use std::io::{stdin, stdout, BufRead, BufWriter, Write};

fn main() {
    let (stdin, stdout) = (stdin(), stdout());
    let (mut stdin, mut stdout) = (stdin.lock(), BufWriter::new(stdout.lock()));

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();

    let parse_int = |s: &str| s.parse::<i32>().unwrap();
    let n = parse_int(buf.trim());

    for _ in 0..n {
        buf.clear();
        stdin.read_line(&mut buf).unwrap();

        let (mut sum, mut min) = (0, 100);

        buf.split_whitespace()
            .map(parse_int)
            .filter(|num| num % 2 == 0)
            .for_each(|num| {
                sum += num;

                if num < min {
                    min = num;
                }
            });

        writeln!(stdout, "{sum} {min}").unwrap();
    }
}
