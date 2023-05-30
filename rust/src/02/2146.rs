use std::collections::VecDeque;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let n = input.next().unwrap();
    let map: Vec<Vec<_>> = (0..n)
        .map(|_| input.by_ref().take(n).map(|num| num == 1).collect())
        .collect();

    let mut visited = vec![vec![false; n]; n];
    let mut min_dist = i32::MAX;

    for y in 0..n {
        for x in 0..n {
            if visited[y][x] || !map[y][x] {
                continue;
            }

            visited[y][x] = true;
            let mut borders = get_borders((y, x), &map, &mut visited);
            let mut bridge_visited = visited.clone();

            'outer: while let Some(((r, c), dist)) = borders.pop_front() {
                for (adj_r, adj_c) in get_adjacents(r, c, n) {
                    if bridge_visited[adj_r][adj_c] {
                        continue;
                    }

                    bridge_visited[adj_r][adj_c] = true;

                    if map[adj_r][adj_c] {
                        min_dist = dist.min(min_dist);
                        break 'outer;
                    }

                    borders.push_back(((adj_r, adj_c), dist + 1));
                }
            }
        }
    }

    println!("{min_dist}");
}

fn get_borders(
    start: (usize, usize),
    map: &[Vec<bool>],
    visited: &mut Vec<Vec<bool>>,
) -> VecDeque<((usize, usize), i32)> {
    let n = map.len();
    let mut borders = VecDeque::new();
    let mut stack = vec![start];

    while let Some((r, c)) = stack.pop() {
        for (adj_r, adj_c) in get_adjacents(r, c, n) {
            if visited[adj_r][adj_c] {
                continue;
            }

            visited[adj_r][adj_c] = true;

            if !map[adj_r][adj_c] {
                borders.push_back(((adj_r, adj_c), 1));
                continue;
            }

            stack.push((adj_r, adj_c));
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
