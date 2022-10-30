use std::collections::HashMap;
use std::io::{stdin, stdout, BufRead, BufWriter, Write};

fn main() {
    let stdin = stdin();
    let stdout = stdout();
    let mut stdin = stdin.lock();
    let mut stdout = BufWriter::new(stdout.lock());

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();

    let n: usize = buf.trim().parse().unwrap();

    let digit_cycle: HashMap<usize, Vec<usize>> = (0..=9)
        .map(|i| {
            let mut cycle = vec![i];
            let mut next = (i * i) % 10;

            loop {
                if cycle.contains(&next) {
                    break;
                }

                cycle.push(next);
                next = (next * i) % 10;
            }

            (i, cycle)
        })
        .collect();

    for _ in 0..n {
        buf.clear();
        stdin.read_line(&mut buf).unwrap();

        if let [a, b] = parse_int_vec(&buf)[..] {
            let cycle = digit_cycle.get(&(a % 10)).unwrap();
            let number = cycle[(b - 1) % cycle.len()];

            writeln!(stdout, "{}", if number == 0 { 10 } else { number }).unwrap();
        }
    }
}

fn parse_int_vec(buf: &String) -> Vec<usize> {
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}
