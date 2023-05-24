use std::collections::VecDeque;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();

    let (height, width) = (
        parse_int(input.next().unwrap()),
        parse_int(input.next().unwrap()),
    );
    let map: Vec<_> = input.map(str::as_bytes).collect();

    let mut visited = vec![vec![[false; 2]; width as usize]; height as usize];
    visited[0][0][0] = true;

    let mut queue = VecDeque::from([((0, 0), 1, false)]);
    let mut min_dist = i32::MAX;

    while let Some(((r, c), dist, has_broken)) = queue.pop_front() {
        if (r, c) == (height - 1, width - 1) {
            min_dist = dist.min(min_dist);
            continue;
        }

        let broken_count = usize::from(has_broken);
        let adjacents = [
            ((r - 1).max(0), c),
            (r, (c - 1).max(0)),
            ((r + 1).min(height - 1), c),
            (r, (c + 1).min(width - 1)),
        ];

        for (adj_r, adj_c) in adjacents {
            let adj = (adj_r as usize, adj_c as usize);

            if visited[adj.0][adj.1][broken_count] {
                continue;
            }

            visited[adj.0][adj.1][broken_count] = true;
            let is_wall = map[adj.0][adj.1] == b'1';

            if is_wall && has_broken {
                continue;
            }

            queue.push_back(((adj_r, adj_c), dist + 1, has_broken || is_wall));
        }
    }

    println!("{}", if min_dist == i32::MAX { -1 } else { min_dist });
}

fn parse_int(buf: &str) -> i16 {
    buf.parse().unwrap()
}
