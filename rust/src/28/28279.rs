use std::collections::VecDeque;
use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();
    let mut input = || input.next().unwrap();
    let mut output = String::new();

    let n: i32 = input().parse().unwrap();
    let mut queue = VecDeque::with_capacity(1_000_000 >> 1);

    for _ in 0..n {
        match input() {
            "1" => {
                queue.push_front(input());
                continue;
            }
            "2" => {
                queue.push_back(input());
                continue;
            }
            "3" => writeln!(output, "{}", queue.pop_front().unwrap_or("-1")),
            "4" => writeln!(output, "{}", queue.pop_back().unwrap_or("-1")),
            "5" => writeln!(output, "{}", queue.len()),
            "6" => writeln!(output, "{}", queue.is_empty() as u8),
            "7" => writeln!(output, "{}", queue.front().unwrap_or(&"-1")),
            "8" => writeln!(output, "{}", queue.back().unwrap_or(&"-1")),
            _ => unreachable!(),
        }
        .unwrap();
    }

    print!("{output}");
}
