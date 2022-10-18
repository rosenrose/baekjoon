use std::collections::HashSet;
use std::io::{stdin, stdout, BufRead, BufWriter, Write};

fn main() {
    let stdin = stdin();
    let stdout = stdout();
    let mut stdin = stdin.lock();
    let mut stdout = BufWriter::new(stdout.lock());

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();

    if let [n, m] = parse_int_vec(&buf)[..] {
        let word_set: HashSet<String> = (0..n)
            .map(|_| {
                buf.clear();
                stdin.read_line(&mut buf).unwrap();

                buf.trim().to_string()
            })
            .collect();

        let mut count = 0;

        for _ in 0..m {
            buf.clear();
            stdin.read_line(&mut buf).unwrap();

            let word = buf.trim();

            if word_set.contains(word) {
                count += 1;
            }
        }

        writeln!(stdout, "{count}").unwrap();
    }
}

fn parse_int_vec(buf: &String) -> Vec<i32> {
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}
