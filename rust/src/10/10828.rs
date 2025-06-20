use std::fmt::Write;
use std::io;

const MAX: usize = 10000;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();
    let mut input = || input.next().unwrap();
    let mut output = String::new();

    let n: i32 = input().parse().unwrap();
    let mut stack = [0; MAX];
    let mut stack_len = 0;

    for _ in 0..n {
        let result = match input() {
            "push" => {
                stack[stack_len] = input().parse().unwrap();
                stack_len += 1;
                continue;
            }
            "pop" => {
                if stack_len == 0 {
                    -1
                } else {
                    stack_len -= 1;
                    stack[stack_len]
                }
            }
            "size" => stack_len as i32,
            "empty" => (stack_len == 0) as i32,
            "top" => {
                if stack_len == 0 {
                    -1
                } else {
                    stack[stack_len - 1]
                }
            }
            _ => unreachable!(),
        };

        writeln!(output, "{result}").unwrap();
    }

    print!("{output}");
}
