use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut output = String::new();
    let (mut left, mut right) = (String::new(), String::new());

    for input in buf.lines().skip(1) {
        for c in input.chars() {
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
        }

        right = right.chars().rev().collect();

        writeln!(output, "{left}{right}").unwrap();

        left.clear();
        right.clear();
    }

    print!("{output}");
}
