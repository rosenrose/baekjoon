use std::fmt::Write;
use std::io;

const MAX: usize = 1_000_000;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut input = || input.next().unwrap();
    let mut output = String::new();

    let n = input();
    let mut stack = [0; MAX];
    let mut stack_len = 0;

    for _ in 0..n {
        let result = match input() {
            1 => {
                stack[stack_len] = input();
                stack_len += 1;
                continue;
            }
            2 => {
                if stack_len == 0 {
                    -1
                } else {
                    stack_len -= 1;
                    stack[stack_len]
                }
            }
            3 => stack_len as i32,
            4 => (stack_len == 0) as i32,
            5 => {
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
