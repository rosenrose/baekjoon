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
        VecDeque::from([((0, 0), Vec::<(i16, i16)>::new(), 0)])
    } else {
        map[0][0] = BackSlash;
        VecDeque::from([((0, 0), Vec::new(), 1)])
    };

    while let Some(((r, c), changes, rotate_count)) = queue.pop_front() {
        for change in changes.iter() {
            map[change.0 as usize][change.1 as usize] =
                if map[change.0 as usize][change.1 as usize] == Slash {
                    BackSlash
                } else {
                    Slash
                };
        }

        if (r, c, map[r as usize][c as usize]) == (height - 1, width - 1, BackSlash) {
            min_count = rotate_count.min(min_count);
        }

        let adjacents = DIRS.map(|dir| (r + dir.0, c + dir.1));

        for &(adj_r, adj_c) in adjacents
            .iter()
            .filter(|&&(adj_r, adj_c)| 0 <= adj_r && adj_r < height && 0 <= adj_c && adj_c < width)
        {
            let adj = (adj_r as usize, adj_c as usize);

            if visited[adj.0][adj.1] {
                continue;
            }

            if can_move((r, c), (adj_r, adj_c), &map) {
                visited[adj.0][adj.1] = true;
                queue.push_front(((adj_r, adj_c), changes.clone(), rotate_count));
            }

            if can_move_after_rotate((r, c), (adj_r, adj_c), &map) {
                let mut new_changes = changes.clone();
                new_changes.push((adj_r, adj_c));

                visited[adj.0][adj.1] = true;
                queue.push_back(((adj_r, adj_c), new_changes, rotate_count + 1));
            }
        }

        for change in changes {
            map[change.0 as usize][change.1 as usize] =
                if map[change.0 as usize][change.1 as usize] == Slash {
                    BackSlash
                } else {
                    Slash
                };
        }
    }

    if min_count == i32::MAX {
        println!("NO SOLUTION");
        return;
    }

    println!("{min_count}");
}

fn can_move(start: (i16, i16), end: (i16, i16), map: &[Vec<Cells>]) -> bool {
    let start_ch = map[start.0 as usize][start.1 as usize];
    let end_ch = map[end.0 as usize][end.1 as usize];
    let dir = (end.0 - start.0, end.1 - start.1);

    matches!(
        (start_ch, end_ch, dir),
        (BackSlash, BackSlash, (-1, -1))
            | (BackSlash, Slash, (-1, 0))
            | (BackSlash, Slash, (0, -1))
            | (BackSlash, Slash, (0, 1))
            | (BackSlash, Slash, (1, 0))
            | (BackSlash, BackSlash, (1, 1))
    ) || matches!(
        (start_ch, end_ch, dir),
        (Slash, BackSlash, (-1, 0))
            | (Slash, Slash, (-1, 1))
            | (Slash, BackSlash, (0, -1))
            | (Slash, BackSlash, (0, 1))
            | (Slash, Slash, (1, -1))
            | (Slash, BackSlash, (1, 0))
    )
}

fn can_move_after_rotate(start: (i16, i16), end: (i16, i16), map: &[Vec<Cells>]) -> bool {
    let start_ch = map[start.0 as usize][start.1 as usize];
    let end_ch = map[end.0 as usize][end.1 as usize];
    let dir = (end.0 - start.0, end.1 - start.1);

    matches!(
        (start_ch, end_ch, dir),
        (BackSlash, Slash, (-1, -1))
            | (BackSlash, BackSlash, (-1, 0))
            | (BackSlash, BackSlash, (0, -1))
            | (BackSlash, BackSlash, (0, 1))
            | (BackSlash, BackSlash, (1, 0))
            | (BackSlash, Slash, (1, 1))
    ) || matches!(
        (start_ch, end_ch, dir),
        (Slash, Slash, (-1, 0))
            | (Slash, BackSlash, (-1, 1))
            | (Slash, Slash, (0, -1))
            | (Slash, Slash, (0, 1))
            | (Slash, BackSlash, (1, -1))
            | (Slash, Slash, (1, 0))
    )
}

fn parse_int(buf: &str) -> i16 {
    buf.parse().unwrap()
}
