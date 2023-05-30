use std::collections::VecDeque;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let n = input.next().unwrap();
    let map: Vec<Vec<_>> = (0..n)
        .map(|_| input.by_ref().take(n).map(|num| num == 1).collect())
        .collect();

    let mut visited = vec![vec![i32::MAX; n]; n];
    let mut min_dist = i32::MAX;

    for y in 0..n {
        for x in 0..n {
            if visited[y][x] != i32::MAX || !map[y][x] {
                continue;
            }

            visited[y][x] = 0;
            let mut borders = get_borders((y, x), &map, &mut visited);

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
    map: &[Vec<bool>],
    visited: &mut Vec<Vec<i32>>,
) -> VecDeque<((usize, usize), i32)> {
    let n = map.len();
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
