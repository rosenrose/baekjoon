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

const WIDTH_MAX: usize = 100;
const HEIGHT_MAX: usize = 100;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();

    let [width, height] = [(); 2].map(|_| input.next().unwrap().parse::<usize>().unwrap());
    let mut lasers = [(0, 0); 2];
    let mut lasers_len = 0;
    let mut map = [[false; WIDTH_MAX]; HEIGHT_MAX];

    for (r, row) in input.enumerate() {
        for (c, ch) in row.char_indices() {
            map[r][c] = match ch {
                '.' => false,
                '*' => true,
                'C' => {
                    lasers[lasers_len] = (r, c);
                    lasers_len += 1;
                    false
                }
                _ => unreachable!(),
            }
        }
    }

    let mut visited = [[[i32::MAX; 4]; WIDTH_MAX]; HEIGHT_MAX];
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
            let next_count = count + (dir != adj_dir) as i32;

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
