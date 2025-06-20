use std::fmt::Write;
use std::io;

const MAX: usize = 1_000_000 + 1;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut input = || input.next().unwrap();
    let mut output = String::new();

    let n = input();
    let mut deque = [0; MAX];
    let mut front = 0;
    let mut back = 1;
    let is_empty = |front: usize, back: usize| (front + 1) % MAX == back;

    for _ in 0..n {
        let result = match input() {
            1 => {
                deque[front] = input();
                front = (front + MAX - 1) % MAX;
                continue;
            }
            2 => {
                deque[back] = input();
                back = (back + 1) % MAX;
                continue;
            }
            3 => {
                if is_empty(front, back) {
                    -1
                } else {
                    front = (front + 1) % MAX;
                    deque[front]
                }
            }
            4 => {
                if is_empty(front, back) {
                    -1
                } else {
                    back = (back + MAX - 1) % MAX;
                    deque[back]
                }
            }
            5 => {
                (if front < back {
                    back - front - 1
                } else {
                    back + (MAX - front) - 1
                }) as i32
            }
            6 => is_empty(front, back) as i32,
            7 => {
                if is_empty(front, back) {
                    -1
                } else {
                    deque[(front + 1) % MAX]
                }
            }
            8 => {
                if is_empty(front, back) {
                    -1
                } else {
                    deque[(back + MAX - 1) % MAX]
                }
            }
            _ => unreachable!(),
        };

        writeln!(output, "{result}").unwrap();
    }

    print!("{output}");
}
