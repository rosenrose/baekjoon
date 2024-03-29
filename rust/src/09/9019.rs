use std::collections::VecDeque;
use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<u16>);
    let mut input = || input.next().unwrap();
    let mut output = String::new();

    const MAX: u16 = 10_000;

    for (a, b) in (0..input()).map(|_| (input(), input())) {
        let mut visited = [None; MAX as usize];
        let mut queue = VecDeque::from([(a, 0)]);

        let steps = 'a: {
            while let Some((num, count)) = queue.pop_front() {
                let next_count = count + 1;
                let adjacents = [
                    ((num << 1) % MAX, 'D'),
                    (if num == 0 { MAX - 1 } else { num - 1 }, 'S'),
                    (rotate_left(num), 'L'),
                    (rotate_right(num), 'R'),
                ];

                for &(adj_num, adj_op) in adjacents.iter().filter(|&&(adj_num, _)| adj_num != a) {
                    if visited[adj_num as usize].is_some() {
                        continue;
                    }

                    visited[adj_num as usize] = Some((num, adj_op));

                    if adj_num == b {
                        break 'a next_count;
                    }

                    queue.push_back((adj_num, next_count));
                }
            }

            unreachable!()
        };

        let mut node = b;
        let mut path = Vec::with_capacity(steps as usize);

        while let Some((parent, op)) = visited[node as usize] {
            path.push(op);
            node = parent;
        }

        writeln!(output, "{}", String::from_iter(path.iter().rev())).unwrap();
    }

    print!("{output}");
}

fn rotate_left(num: u16) -> u16 {
    let mut digits = to_digits(num);
    digits.rotate_left(1);

    to_int(digits)
}

fn rotate_right(num: u16) -> u16 {
    let mut digits = to_digits(num);
    digits.rotate_right(1);

    to_int(digits)
}

fn to_digits(mut num: u16) -> [u8; 4] {
    let mut digits = [0; 4];

    for digit in digits.iter_mut().rev() {
        if num == 0 {
            break;
        }

        *digit = (num % 10) as u8;
        num /= 10;
    }

    digits
}

fn to_int(digits: [u8; 4]) -> u16 {
    digits.iter().fold(0, |acc, &digit| acc * 10 + digit as u16)
}
