use std::collections::VecDeque;
use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();
    let mut input = || input.next().unwrap();
    let mut output = String::new();

    let n = input().parse().unwrap();
    let mut queue = VecDeque::new();

    for _ in 0..n {
        let result = match input() {
            "pop" => queue.pop_front().unwrap_or("-1"),
            "size" => {
                writeln!(output, "{}", queue.len()).unwrap();
                continue;
            }
            "empty" => {
                if queue.is_empty() {
                    "1"
                } else {
                    "0"
                }
            }
            "front" => queue.front().unwrap_or(&"-1"),
            "back" => queue.back().unwrap_or(&"-1"),
            _ => {
                queue.push_back(input());
                continue;
            }
        };

        writeln!(output, "{result}").unwrap();
    }

    print!("{output}");
}
