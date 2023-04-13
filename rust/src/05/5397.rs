use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut output = String::new();
    let (mut left, mut right) = (String::new(), String::new());

    for input in buf.lines().skip(1) {
        for ch in input.chars() {
            match ch {
                '-' => {
                    left.pop();
                }
                '<' => {
                    if let Some(ch) = left.pop() {
                        right.push(ch);
                    }
                }
                '>' => {
                    if let Some(c) = right.pop() {
                        left.push(ch);
                    }
                }
                _ => left.push(ch),
            };
        }

        right = right.chars().rev().collect();

        writeln!(output, "{left}{right}").unwrap();

        left.clear();
        right.clear();
    }

    print!("{output}");
}
