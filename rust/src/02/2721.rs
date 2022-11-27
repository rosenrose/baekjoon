use std::io::{stdin, stdout, BufWriter, Read, Write};

fn main() {
    let (stdin, stdout) = (stdin(), stdout());
    let (mut stdin, mut stdout) = (stdin.lock(), BufWriter::new(stdout.lock()));

    let mut buf = String::new();
    stdin.read_to_string(&mut buf).unwrap();

    let t = |n: usize| n * (n + 1) / 2;
    let mut t_sum = vec![0];

    while t_sum.len() < 301 {
        let len = t_sum.len();
        t_sum.push(t_sum.last().unwrap() + len * t(len + 1));
    }

    for line in buf.lines().skip(1) {
        writeln!(stdout, "{}", t_sum[parse_int(line)]).unwrap();
    }
}

fn parse_int(buf: &str) -> usize {
    buf.trim().parse().unwrap()
}
