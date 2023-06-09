#[derive(Copy, Clone, PartialEq, Debug)]
enum Dirs {
    Up,
    Down,
    Left,
    Right,
}

use std::collections::VecDeque;
use std::io;
use Dirs::*;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();

    let (width, height) = (
        parse_int(input.next().unwrap()),
        parse_int(input.next().unwrap()),
    );
    let mut lasers = Vec::with_capacity(2);
    let map: Vec<Vec<_>> = input
        .enumerate()
        .map(|(r, row)| {
            row.char_indices()
                .map(|(c, ch)| match ch {
                    '.' => false,
                    '*' => true,
                    'C' => {
                        lasers.push((r, c));
                        false
                    }
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();

    let mut visited = vec![vec![[i32::MAX; 4]; width]; height];
    let mut queue = VecDeque::from_iter(get_adjacents(lasers[0], width, height).iter().filter_map(
        |&(coord, dir)| {
            (coord != lasers[0] && !map[coord.0][coord.1]).then(|| {
                visited[coord.0][coord.1][dir as usize] = 0;
                (coord, dir, 0)
            })
        },
    ));

    let mut min_count = i32::MAX;

    while let Some(((r, c), dir, count)) = queue.pop_front() {
        if (r, c) == lasers[1] {
            min_count = count.min(min_count);
            continue;
        }

        let adjacents =
            get_adjacents((r, c), width, height)
                .into_iter()
                .filter(|&(adj_coord, adj_dir)| {
                    !map[adj_coord.0][adj_coord.1]
                        && match dir {
                            Up => adj_dir != Down,
                            Down => adj_dir != Up,
                            Left => adj_dir != Right,
                            Right => adj_dir != Left,
                        }
                });

        for ((adj_r, adj_c), adj_dir) in adjacents {
            let next_count = count + i32::from(dir != adj_dir);

            if visited[adj_r][adj_c][adj_dir as usize] <= next_count {
                continue;
            }

            visited[adj_r][adj_c][adj_dir as usize] = next_count;
            queue.push_back(((adj_r, adj_c), adj_dir, next_count));
        }
    }

    println!("{min_count}");
}

fn get_adjacents(
    (r, c): (usize, usize),
    width: usize,
    height: usize,
) -> [((usize, usize), Dirs); 4] {
    [
        ((r.saturating_sub(1), c), Up),
        ((r, c.saturating_sub(1)), Left),
        (((r + 1).min(height - 1), c), Down),
        ((r, (c + 1).min(width - 1)), Right),
    ]
}

fn parse_int(buf: &str) -> usize {
    buf.parse().unwrap()
}
