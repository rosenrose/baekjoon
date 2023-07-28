use std::collections::VecDeque;
use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();
    let mut input = || input.next().unwrap();
    let mut output = String::new();

    let n = parse_int(input());
    let mut queue = VecDeque::new();

    for _ in 0..n {
        let result = match input() {
            "push" => {
                queue.push_back(parse_int(input()));
                continue;
            }
            "pop" => queue.pop_front().unwrap_or(-1),
            "size" => queue.len() as i32,
            "empty" => queue.is_empty() as i32,
            "front" => *queue.front().unwrap_or(&-1),
            "back" => *queue.back().unwrap_or(&-1),
            _ => unreachable!(),
        };

        writeln!(output, "{result}").unwrap();
    }

    print!("{output}");
}

fn parse_int(buf: &str) -> i32 {
    buf.parse().unwrap()
}
