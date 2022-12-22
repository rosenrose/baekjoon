use std::collections::VecDeque;
use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut input = buf.split_ascii_whitespace();

    let (n, m) = (
        parse_int(input.next().unwrap()),
        parse_int(input.next().unwrap()),
    );
    let mut maze: Vec<Vec<_>> = input
        .map(|row| row.as_bytes().iter().map(|c| (*c, 1)).collect())
        .collect();

    let mut queue = VecDeque::from([(0_usize, 0_usize, 1)]);

    while !queue.is_empty() {
        let (x, y, step) = queue.pop_front().unwrap();

        let adjacent = [
            (x.saturating_sub(1), y),
            (x, y.saturating_sub(1)),
            (x + 1, y),
            (x, y + 1),
        ];
        let adjacent = adjacent.iter().filter(|(ad_x, ad_y)| {
            (*ad_x, *ad_y) != (x, y) && (0..m).contains(ad_x) && (0..n).contains(ad_y)
        });

        for &(x, y) in adjacent {
            if maze[y][x].0 == '0' as u8 || maze[y][x].1 != 1 {
                continue;
            }

            maze[y][x].1 = step + 1;
            queue.push_back((x, y, step + 1));
        }
    }

    println!("{}", maze[n - 1][m - 1].1);
}

fn parse_int(buf: &str) -> usize {
    buf.parse().unwrap()
}
