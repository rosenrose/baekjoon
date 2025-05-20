use std::collections::VecDeque;
use std::io;

const WIDTH_MAX: usize = 1000;
const HEIGHT_MAX: usize = 1000;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();

    let [height, width] = [(); 2].map(|_| input.next().unwrap().parse::<i16>().unwrap());
    let mut map = [[0; WIDTH_MAX]; HEIGHT_MAX];

    for (r, row) in input.map(str::as_bytes).enumerate() {
        for (c, &num) in row.iter().enumerate() {
            map[r][c] = num;
        }
    }

    let mut visited = [[[false; 2]; WIDTH_MAX]; HEIGHT_MAX];
    visited[0][0][0] = true;

    let mut queue = VecDeque::from([((0, 0), 1, false)]);
    let mut min_dist = i32::MAX;

    while let Some(((r, c), dist, has_broken)) = queue.pop_front() {
        if (r, c) == (height - 1, width - 1) {
            min_dist = dist.min(min_dist);
            continue;
        }

        let new_dist = dist + 1;
        let broken_count = has_broken as usize;
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

            let can_break = is_wall && !has_broken;

            if can_break {
                visited[adj.0][adj.1][can_break as usize] = true;
                queue.push_back(((adj_r, adj_c), new_dist, can_break));
            } else {
                queue.push_back(((adj_r, adj_c), new_dist, has_broken));
            }
        }
    }

    println!("{}", if min_dist == i32::MAX { -1 } else { min_dist });
}
