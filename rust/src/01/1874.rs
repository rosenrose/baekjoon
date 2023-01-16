use std::cmp::Ordering;
use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.lines().flat_map(str::parse);
    let mut output = String::new();

    let mut stack_nums = 1..=input.next().unwrap();
    let mut stack = Vec::new();
    let mut ops = Vec::new();

    for input_num in input {
        match input_num.cmp(stack.last().unwrap_or(&0)) {
            Ordering::Greater => {
                while let Some(stack_num) = stack_nums.next() {
                    stack.push(stack_num);
                    ops.push('+');

                    if stack_num == input_num {
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
