use std::fmt::Write;
use std::io;

const MAX: usize = 10000 + 1;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();
    let mut input = || input.next().unwrap();
    let mut output = String::new();

    let n: i32 = input().parse().unwrap();
    let mut queue = [0; MAX];
    let mut front = 0;
    let mut back = 1;
    let len = |front: usize, back: usize| {
        if front < back {
            back - front - 1
        } else {
            back + (MAX - front) - 1
        }
    };

    for _ in 0..n {
        let result = match input() {
            "push" => {
                queue[back] = input().parse().unwrap();
                back = (back + 1) % MAX;
                continue;
            }
            "pop" => {
                if len(front, back) == 0 {
                    -1
                } else {
                    front = (front + 1) % MAX;
                    queue[front]
                }
            }
            "size" => len(front, back) as i32,
            "empty" => (len(front, back) == 0) as i32,
            "front" => {
                if len(front, back) == 0 {
                    -1
                } else {
                    queue[(front + 1) % MAX]
                }
            }
            "back" => {
                if len(front, back) == 0 {
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
