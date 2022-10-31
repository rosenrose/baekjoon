use std::collections::HashSet;
use std::io::{stdin, stdout, BufRead, BufWriter, Write};

fn main() {
    let (stdin, stdout) = (stdin(), stdout());
    let (mut stdin, mut stdout) = (stdin.lock(), BufWriter::new(stdout.lock()));

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();

    if let [n, m] = parse_int_vec(&buf)[..] {
        let not_heard: HashSet<String> = (0..n)
            .map(|_| {
                buf.clear();
                stdin.read_line(&mut buf).unwrap();

                buf.trim().to_string()
            })
            .collect();

        let mut not_heard_seen = Vec::new();

        for _ in 0..m {
            buf.clear();
            stdin.read_line(&mut buf).unwrap();
            let not_seen = buf.trim().to_string();

            if not_heard.contains(&not_seen) {
                not_heard_seen.push(not_seen);
            }
        }

        not_heard_seen.sort();

        writeln!(stdout, "{}", not_heard_seen.len()).unwrap();
        writeln!(stdout, "{}", not_heard_seen.join("\n")).unwrap();
    }
}

fn parse_int_vec(buf: &String) -> Vec<i32> {
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}
