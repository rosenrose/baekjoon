use std::cmp::Ordering;
use std::io::{stdin, stdout, BufWriter, Read, Write};

fn main() {
    let (stdin, stdout) = (stdin(), stdout());
    let (mut stdin, mut stdout) = (stdin.lock(), BufWriter::new(stdout.lock()));

    let mut buf = String::new();
    stdin.read_to_string(&mut buf).unwrap();

    let mut lines = buf.lines();
    let n = parse_int(lines.next().unwrap());

    let mut nums = 1..=n;
    let mut stack = Vec::new();
    let mut ops = Vec::new();

    for line in lines {
        let input = parse_int(line);

        match input.cmp(stack.last().unwrap_or(&0)) {
            Ordering::Greater => {
                loop {
                    let num = nums.next().unwrap();

                    stack.push(num);
                    ops.push('+');

                    if num == input {
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
                writeln!(stdout, "NO").unwrap();
                return;
            }
        }
    }

    for op in ops {
        writeln!(stdout, "{op}").unwrap();
    }
}

fn parse_int(buf: &str) -> i32 {
    buf.parse().unwrap()
}
