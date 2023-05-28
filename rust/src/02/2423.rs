const DIRS: [(i16, i16); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

#[derive(Copy, Clone, PartialEq)]
enum Cells {
    Slash,
    BackSlash,
}

use std::collections::VecDeque;
use std::io;
use Cells::*;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();

    let (height, width) = (
        parse_int(input.next().unwrap()),
        parse_int(input.next().unwrap()),
    );
    let mut map: Vec<Vec<_>> = input
        .map(|row| {
            row.chars()
                .map(|ch| match ch {
                    '/' => Slash,
                    '\\' => BackSlash,
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();

    let mut visited = vec![vec![false; width as usize]; height as usize];
    visited[0][0] = true;

    let mut min_count = i32::MAX;
    let mut queue = if map[0][0] == BackSlash {
        VecDeque::from([((0, 0), 0)])
    } else {
        map[0][0] = BackSlash;
        VecDeque::from([((0, 0), 1)])
    };

    while let Some(((r, c), rotate_count)) = queue.pop_front() {
        let cur_ch = map[r as usize][c as usize];

        if (r, c, cur_ch) == (height - 1, width - 1, BackSlash) {
            min_count = rotate_count.min(min_count);
            continue;
        }

        let adjacents = DIRS.map(|dir| ((r + dir.0, c + dir.1), dir));

        for &((adj_r, adj_c), adj_dir) in adjacents.iter().filter(|&&((adj_r, adj_c), _)| {
            0 <= adj_r && adj_r < height && 0 <= adj_c && adj_c < width
        }) {
            let adj = (adj_r as usize, adj_c as usize);
            let adj_ch = map[adj.0][adj.1];

            if visited[adj.0][adj.1] {
                continue;
            }

            if can_move(cur_ch, adj_ch, adj_dir) {
                visited[adj.0][adj.1] = true;
                queue.push_front(((adj_r, adj_c), rotate_count));
            }

            if can_move_after_rotate(cur_ch, adj_ch, adj_dir) {
                map[adj.0][adj.1] = if adj_ch == Slash { BackSlash } else { Slash };

                visited[adj.0][adj.1] = true;
                queue.push_back(((adj_r, adj_c), rotate_count + 1));
            }
        }
    }

    if min_count == i32::MAX {
        println!("NO SOLUTION");
        return;
    }

    println!("{min_count}");
}

fn can_move(start_ch: Cells, end_ch: Cells, dir: (i16, i16)) -> bool {
    (is_diagonal(start_ch, dir) && start_ch == end_ch) || (is_cross(dir) && start_ch != end_ch)
}

fn can_move_after_rotate(start_ch: Cells, end_ch: Cells, dir: (i16, i16)) -> bool {
    (is_diagonal(start_ch, dir) && start_ch != end_ch) || (is_cross(dir) && start_ch == end_ch)
}

fn is_diagonal(ch: Cells, dir: (i16, i16)) -> bool {
    (ch == BackSlash && matches!(dir, (-1, -1) | (1, 1)))
        || (ch == Slash && matches!(dir, (-1, 1) | (1, -1)))
}

fn is_cross(dir: (i16, i16)) -> bool {
    matches!(dir, (-1, 0) | (0, -1) | (0, 1) | (1, 0))
}

fn parse_int(buf: &str) -> i16 {
    buf.parse().unwrap()
}
