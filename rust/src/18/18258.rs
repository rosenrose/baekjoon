use std::fmt::Write;
use std::io;

const MAX: usize = 2_000_000 + 1;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();
    let mut input = || input.next().unwrap();
    let mut output = String::new();

    let n: i32 = input().parse().unwrap();
    let mut queue = [0; MAX];
    let mut front = 0;
    let mut back = 1;
    let is_empty = |front: usize, back: usize| (front + 1) % MAX == back;

    for _ in 0..n {
        let result = match input() {
            "push" => {
                queue[back] = input().parse().unwrap();
                back = (back + 1) % MAX;
                continue;
            }
            "pop" => {
                if is_empty(front, back) {
                    -1
                } else {
                    front = (front + 1) % MAX;
                    queue[front]
                }
            }
            "size" => {
                (if front < back {
                    back - front - 1
                } else {
                    back + (MAX - front) - 1
                }) as i32
            }
            "empty" => is_empty(front, back) as i32,
            "front" => {
                if is_empty(front, back) {
                    -1
                } else {
                    queue[(front + 1) % MAX]
                }
            }
            "back" => {
                if is_empty(front, back) {
                    -1
                } else {
                    queue[(back + MAX - 1) % MAX]
                }
            }
            _ => unreachable!(),
        };

        writeln!(output, "{result}").unwrap();
    }

    print!("{output}");
}
