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

    for line in lines {
        let mut command = line.split_whitespace();

        match command.next().unwrap() {
            "L" if !left.is_empty() => right.push(left.pop().unwrap()),
            "D" if !right.is_empty() => left.push(right.pop().unwrap()),
            "B" if !left.is_empty() => {
                left.pop();
            }
            "P" => {
                let letter = command.next().unwrap().chars().next().unwrap();
                left.push(letter);
            }
            _ => (),
        };
    }

    right = right.chars().rev().collect();

    writeln!(stdout, "{left}{right}").unwrap();
}
