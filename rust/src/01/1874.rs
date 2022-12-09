use std::cmp::Ordering;
use std::fmt::Write;
use std::io::{stdin, Read};

fn main() {
    let stdin = stdin();
    let mut stdin = stdin.lock();

    let mut buf = String::new();
    stdin.read_to_string(&mut buf).unwrap();

    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());
    let mut output = String::new();

    let mut nums = 1..=input.next().unwrap();
    let mut stack = Vec::new();
    let mut ops = Vec::new();

    for num in input {
        match num.cmp(stack.last().unwrap_or(&0)) {
            Ordering::Greater => {
                loop {
                    let temp = nums.next().unwrap();

                    stack.push(temp);
                    ops.push('+');

                    if temp == num {
                        break;
                    }
                }

                stack.pop();
                ops.push('-');
            }
            Ordering::Equal => {
                stack.pop();
                ops.push('-');
            }
            Ordering::Less => {
                print!("NO");
                return;
            }
        }
    }

    for op in ops {
        writeln!(output, "{op}").unwrap();
    }

    print!("{output}");
}
