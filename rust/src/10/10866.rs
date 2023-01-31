use std::collections::VecDeque;
use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();
    let mut input = || input.next().unwrap();
    let mut output = String::new();

    let n = parse_int(input());
    let mut deque = VecDeque::new();

    for _ in 0..n {
        let result = match input() {
            "push_front" => {
                deque.push_front(parse_int(input()));
                continue;
            }
            "push_back" => {
                deque.push_back(parse_int(input()));
                continue;
            }
            "pop_front" => deque.pop_front().unwrap_or(-1),
            "pop_back" => deque.pop_back().unwrap_or(-1),
            "size" => deque.len() as i32,
            "empty" => {
                if deque.is_empty() {
                    1
                } else {
                    0
                }
            }
            "front" => *deque.front().unwrap_or(&-1),
            "back" => *deque.back().unwrap_or(&-1),
            _ => Default::default(),
        };

        writeln!(output, "{result}").unwrap();
    }

    print!("{output}");
}

fn parse_int(buf: &str) -> i32 {
    buf.parse().unwrap()
}
