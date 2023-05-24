use std::collections::VecDeque;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();
    let mut it = input.by_ref().take(3).flat_map(str::parse::<i16>);

    let (height, width, k) = (it.next().unwrap(), it.next().unwrap(), it.next().unwrap());
    let map: Vec<_> = input.map(str::as_bytes).collect();

    let mut visited = vec![vec![[[false; 2]; 11]; width as usize]; height as usize];
    visited[0][0][0][1] = true;

    let mut queue = VecDeque::from([((0, 0), 1, 0, true)]);
    let mut min_dist = i32::MAX;

    while let Some(((r, c), dist, broken_count, is_day)) = queue.pop_front() {
        if (r, c) == (height - 1, width - 1) {
            min_dist = dist.min(min_dist);
            continue;
        }

        let day_idx = usize::from(is_day);
        let adjacents = [
            ((r - 1).max(0), c),
            (r, (c - 1).max(0)),
            ((r + 1).min(height - 1), c),
            (r, (c + 1).min(width - 1)),
        ];

        for (adj_r, adj_c) in adjacents {
            let adj = (adj_r as usize, adj_c as usize);

            if visited[adj.0][adj.1][broken_count as usize][day_idx] {
                continue;
            }

            visited[adj.0][adj.1][broken_count as usize][day_idx] = true;
            let is_wall = map[adj.0][adj.1] == b'1';

            if is_wall && (broken_count == k as u8 || !is_day) {
                continue;
            }

            queue.push_back((
                (adj_r, adj_c),
                dist + 1,
                broken_count + u8::from(is_day && is_wall),
                !is_day,
            ));
        }

        if !is_day {
            queue.push_back(((r, c), dist + 1, broken_count, !is_day));
        }
    }

    println!("{}", if min_dist == i32::MAX { -1 } else { min_dist });
}
