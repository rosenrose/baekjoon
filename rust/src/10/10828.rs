use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();
    let mut input = || input.next().unwrap();
    let mut output = String::new();

    let n = parse_int(input());
    let mut stack = Vec::new();

    for _ in 0..n {
        let result = match input() {
            "push" => {
                stack.push(parse_int(input()));
                continue;
            }
            "pop" => stack.pop().unwrap_or(-1),
            "size" => stack.len() as i32,
            "empty" => {
                if stack.is_empty() {
                    1
                } else {
                    0
                }
            }
            "top" => *stack.last().unwrap_or(&-1),
            _ => unreachable!(),
        };

        writeln!(output, "{result}").unwrap();
    }

    print!("{output}");
}

fn parse_int(buf: &str) -> i32 {
    buf.parse().unwrap()
}
