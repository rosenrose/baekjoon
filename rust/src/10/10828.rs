use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();
    let mut output = String::new();

    let n = parse_int(input.next().unwrap());
    let mut stack = Vec::new();

    for _ in 0..n {
        let result = match input.next().unwrap() {
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
            _ => {
                stack.push(parse_int(input.next().unwrap()));
                continue;
            }
        };

        writeln!(output, "{result}").unwrap();
    }

    print!("{output}");
}

fn parse_int(buf: &str) -> i32 {
    buf.parse().unwrap()
}
