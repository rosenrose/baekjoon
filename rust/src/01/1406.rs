use std::io::{stdin, stdout, BufWriter, Read, Write};

fn main() {
    let (stdin, stdout) = (stdin(), stdout());
    let (mut stdin, mut stdout) = (stdin.lock(), BufWriter::new(stdout.lock()));

    let mut buf = String::new();
    stdin.read_to_string(&mut buf).unwrap();
    let mut lines = buf.lines();

    let mut left = lines.next().unwrap().to_string();
    let mut right = String::new();
    lines.next();

    lines.for_each(|line| {
        let mut command = line.split_whitespace();

        match command.next().unwrap() {
            "L" => {
                if let Some(c) = left.pop() {
                    right.push(c);
                }
            }
            "D" => {
                if let Some(c) = right.pop() {
                    left.push(c);
                }
            }
            "B" => {
                left.pop();
            }
            "P" => {
                let c = command.next().unwrap().chars().next().unwrap();
                left.push(c);
            }
            _ => (),
        };
    });

    right = right.chars().rev().collect();

    writeln!(stdout, "{left}{right}").unwrap();
}
