use std::io::{self, BufRead, Write};

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let (mut stdin, mut stdout) = (stdin.lock(), io::BufWriter::new(stdout.lock()));

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();

    let n: i32 = buf.trim_end().parse().unwrap();
    const MAX_NUM: usize = 10000;

    let mut count = [0; MAX_NUM + 1];
    let (mut max, mut min) = (0, MAX_NUM);

    for _ in 0..n {
        buf.clear();
        stdin.read_line(&mut buf).unwrap();

        let num: usize = buf.trim_end().parse().unwrap();
        count[num] += 1;

        min = num.min(min);
        max = num.max(max);
    }

    for num in min..=max {
        for _ in 0..count[num] {
            writeln!(stdout, "{num}").unwrap();
        }
    }
}
