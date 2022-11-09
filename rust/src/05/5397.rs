use std::io::{stdin, stdout, BufWriter, Read, Write};

fn main() {
    let (stdin, stdout) = (stdin(), stdout());
    let (mut stdin, mut stdout) = (stdin.lock(), BufWriter::new(stdout.lock()));

    let mut buf = String::new();
    stdin.read_to_string(&mut buf).unwrap();

    let mut lines = buf.lines();
    lines.next();

    let mut left = String::new();
    let mut right = String::new();

    lines.for_each(|line| {
        line.chars().for_each(|c| {
            match c {
                '-' => {
                    left.pop();
                }
                '<' => {
                    if let Some(c) = left.pop() {
                        right.push(c);
                    }
                }
                '>' => {
                    if let Some(c) = right.pop() {
                        left.push(c);
                    }
                }
                _ => left.push(c),
            };
        });

        right = right.chars().rev().collect();

        writeln!(stdout, "{left}{right}").unwrap();

        left.clear();
        right.clear();
    });
}
