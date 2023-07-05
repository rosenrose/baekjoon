use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();
    let mut input = || input.next().unwrap();
    let mut output = String::new();

    let n: i32 = input().parse().unwrap();
    let mut stack = Vec::with_capacity(1_000_000);

    for _ in 0..n {
        match input() {
            "1" => {
                stack.push(input());
                continue;
            }
            "2" => writeln!(output, "{}", stack.pop().unwrap_or("-1")),
            "3" => writeln!(output, "{}", stack.len()),
            "4" => writeln!(output, "{}", u8::from(stack.is_empty())),
            "5" => writeln!(output, "{}", stack.last().unwrap_or(&"-1")),
            _ => unreachable!(),
        }
        .unwrap();
    }

    print!("{output}");
}
