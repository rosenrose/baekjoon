use std::collections::VecDeque;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();
    let mut input = || input.next().unwrap();

    let (height, width) = (parse_int(input()), parse_int(input()));
    let (x1, y1, x2, y2) = (
        parse_int(input()) - 1,
        parse_int(input()) - 1,
        parse_int(input()) - 1,
        parse_int(input()) - 1,
    );
    let (x1, y1, x2, y2) = (y1, x1, y2, x2);
    let map: Vec<_> = (0..height).map(|_| input().as_bytes()).collect();

    let mut visited = vec![vec![false; width]; height];
    visited[y1][x1] = true;

    let mut min_count = i32::MAX;
    let mut queue = VecDeque::from([((y1, x1), 1)]);

    while let Some(((r, c), break_count)) = queue.pop_front() {
        if (r, c) == (y2, x2) {
            min_count = break_count.min(min_count);
            continue;
        }

        let adjacents = [
            (r.saturating_sub(1), c),
            (r, c.saturating_sub(1)),
            ((r + 1).min(height - 1), c),
            (r, (c + 1).min(width - 1)),
        ];

        for (adj_r, adj_c) in adjacents {
            if visited[adj_r][adj_c] {
                continue;
            }

            visited[adj_r][adj_c] = true;

            if map[adj_r][adj_c] == b'1' {
                queue.push_back(((adj_r, adj_c), break_count + 1));
            } else {
                queue.push_front(((adj_r, adj_c), break_count));
            }
        }
    }

    println!("{min_count}");
}

fn parse_int(buf: &str) -> usize {
    buf.parse().unwrap()
}
