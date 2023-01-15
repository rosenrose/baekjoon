use std::cmp::Ordering;
use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
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
