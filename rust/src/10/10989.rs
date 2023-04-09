use std::io::{self, BufRead, Write};

fn main() {
    let mut stdin = io::stdin().lock();
    let mut stdout = io::BufWriter::new(io::stdout().stdout.lock());

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

        (min, max) = (num.min(min), num.max(max));
    }

    for num in min..=max {
        for _ in 0..count[num] {
            writeln!(stdout, "{num}").unwrap();
        }
    }
}
