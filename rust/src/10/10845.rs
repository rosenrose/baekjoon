use std::collections::VecDeque;
use std::fmt::Write;
use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut input = buf.split_ascii_whitespace();
    let mut output = String::new();

    let n = parse_int(input.next().unwrap());
    let mut queue = VecDeque::new();

    for _ in 0..n {
        let result = match input.next().unwrap() {
            "pop" => queue.pop_front().unwrap_or(-1),
            "size" => queue.len() as i32,
            "empty" => {
                if queue.is_empty() {
                    1
                } else {
                    0
                }
            }
            "front" => *queue.front().unwrap_or(&-1),
            "back" => *queue.back().unwrap_or(&-1),
            _ => {
                queue.push_back(parse_int(input.next().unwrap()));
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
