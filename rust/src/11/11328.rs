use std::collections::HashMap;
use std::io::{stdin, stdout, BufRead, BufWriter, Write};

fn main() {
    let (stdin, stdout) = (stdin(), stdout());
    let (mut stdin, mut stdout) = (stdin.lock(), BufWriter::new(stdout.lock()));

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();

    let n: i32 = buf.trim().parse().unwrap();

    'outer: for _ in 0..n {
        buf.clear();
        stdin.read_line(&mut buf).unwrap();

        if let [a, b] = parse_str_vec(&buf)[..] {
            if a.len() != b.len() {
                writeln!(stdout, "Impossible").unwrap();
                continue 'outer;
            }

            let mut a_counts = HashMap::new();

            for letter in a.chars() {
                a_counts.entry(letter).and_modify(|c| *c += 1).or_insert(1);
            }

            let mut b_counts = HashMap::new();

            for letter in b.chars() {
                if !a_counts.contains_key(&letter) {
                    writeln!(stdout, "Impossible").unwrap();
                    continue 'outer;
                }

                b_counts.entry(letter).and_modify(|c| *c += 1).or_insert(1);
            }

            writeln!(
                stdout,
                "{}",
                if a_counts == b_counts {
                    "Possible"
                } else {
                    "Impossible"
                }
            )
            .unwrap();
        }
    }
}

fn parse_str_vec(buf: &String) -> Vec<&str> {
    buf.split_whitespace().collect()
}
