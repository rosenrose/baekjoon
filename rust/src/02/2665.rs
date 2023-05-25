use std::collections::VecDeque;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();

    let n: usize = input.next().unwrap().parse().unwrap();
    let map: Vec<_> = input.map(str::as_bytes).collect();

    let mut visited = vec![vec![false; n]; n];
    visited[0][0] = true;

    let mut min_count = i32::MAX;
    let mut queue = VecDeque::from([((0, 0), 0)]);

    while let Some(((r, c), break_count)) = queue.pop_front() {
        if (r, c) == (n - 1, n - 1) {
            min_count = break_count.min(min_count);
            continue;
        }

        let adjacents = [
            (r.saturating_sub(1), c),
            (r, c.saturating_sub(1)),
            ((r + 1).min(n - 1), c),
            (r, (c + 1).min(n - 1)),
        ];

        for (adj_r, adj_c) in adjacents {
            if visited[adj_r][adj_c] {
                continue;
            }

            visited[adj_r][adj_c] = true;

            if map[adj_r][adj_c] == b'0' {
                queue.push_back(((adj_r, adj_c), break_count + 1));
            } else {
                queue.push_front(((adj_r, adj_c), break_count));
            }
        }
    }

    println!("{min_count}");
}
