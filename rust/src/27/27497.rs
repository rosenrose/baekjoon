use std::collections::VecDeque;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();
    let mut input = || input.next().unwrap();

    let n: usize = input().parse().unwrap();
    let mut queue = VecDeque::with_capacity(n >> 1);
    let mut stack = Vec::with_capacity(n >> 1);

    for _ in 0..n {
        let op = input().as_bytes()[0];

        match op {
            b'1' => {
                queue.push_back(input().as_bytes()[0]);
                stack.push(op)
            }
            b'2' => {
                queue.push_front(input().as_bytes()[0]);
                stack.push(op)
            }
            b'3' => match stack.pop() {
                Some(b'1') => {
                    queue.pop_back();
                }
                Some(b'2') => {
                    queue.pop_front();
                }
                _ => (),
            },
            _ => unreachable!(),
        }
    }

    if queue.is_empty() {
        println!("0");
        return;
    }

    println!("{}", String::from_utf8(Vec::from_iter(queue)).unwrap());
}
