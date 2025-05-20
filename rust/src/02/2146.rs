use std::collections::VecDeque;
use std::io;

const MAX: usize = 100;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let n = input.next().unwrap();
    let mut map = [[false; MAX]; MAX];

    for r in 0..n {
        for (c, num) in input.by_ref().take(n).enumerate() {
            map[r][c] = num == 1;
        }
    }

    let mut visited = [[i32::MAX; MAX]; MAX];
    let mut min_dist = i32::MAX;

    for y in 0..n {
        for x in 0..n {
            if visited[y][x] != i32::MAX || !map[y][x] {
                continue;
            }

            visited[y][x] = 0;
            let mut borders = get_borders((y, x), &map, n, &mut visited);

            'outer: while let Some(((r, c), dist)) = borders.pop_front() {
                let new_dist = dist + 1;

                for (adj_r, adj_c) in get_adjacents(r, c, n) {
                    if visited[adj_r][adj_c] <= new_dist {
                        continue;
                    }

                    visited[adj_r][adj_c] = new_dist;

                    if map[adj_r][adj_c] {
                        min_dist = dist.min(min_dist);
                        break 'outer;
                    }

                    borders.push_back(((adj_r, adj_c), new_dist));
                }
            }
        }
    }

    println!("{min_dist}");
}

fn get_borders(
    start: (usize, usize),
    map: &[[bool; MAX]],
    n: usize,
    visited: &mut [[i32; MAX]],
) -> VecDeque<((usize, usize), i32)> {
    let mut borders = VecDeque::new();
    let mut stack = vec![start];

    while let Some((r, c)) = stack.pop() {
        for (adj_r, adj_c) in get_adjacents(r, c, n) {
            if visited[adj_r][adj_c] <= 1 {
                continue;
            }

            if map[adj_r][adj_c] {
                visited[adj_r][adj_c] = 0;
                stack.push((adj_r, adj_c));
            } else {
                visited[adj_r][adj_c] = 1;
                borders.push_back(((adj_r, adj_c), 1));
            }
        }
    }

    borders
}

fn get_adjacents(r: usize, c: usize, size: usize) -> [(usize, usize); 4] {
    [
        (r.saturating_sub(1), c),
        (r, c.saturating_sub(1)),
        ((r + 1).min(size - 1), c),
        (r, (c + 1).min(size - 1)),
    ]
}
