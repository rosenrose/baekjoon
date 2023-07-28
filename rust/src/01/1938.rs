use std::collections::VecDeque;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let (mut log_coords, mut end_coords) = (Vec::with_capacity(3), Vec::with_capacity(3));

    let map: Vec<Vec<_>> = buf
        .lines()
        .skip(1)
        .enumerate()
        .map(|(r, row)| {
            row.char_indices()
                .map(|(c, ch)| match ch {
                    '0' => false,
                    '1' => true,
                    'B' => {
                        log_coords.push((r as i32, c as i32));
                        false
                    }
                    'E' => {
                        end_coords.push((r as i32, c as i32));
                        false
                    }
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();
    let log = (log_coords[1], log_coords[0].0 + 1 == log_coords[1].0);
    let end = (end_coords[1], end_coords[0].0 + 1 == end_coords[1].0);

    println!("{}", simulate(map, log, end).unwrap_or(0));
}

fn simulate(
    map: Vec<Vec<bool>>,
    (log_center, is_log_vertical): ((i32, i32), bool),
    end: ((i32, i32), bool),
) -> Option<i32> {
    let n = map.len() as i32;
    let mut visited = vec![vec![[false; 2]; n as usize]; n as usize];
    visited[log_center.0 as usize][log_center.1 as usize][is_log_vertical as usize] = true;

    let is_placeable = |r: i32, c: i32, is_vertical: bool| {
        if is_vertical {
            (1 <= r && r < n - 1 && 0 <= c && c < n)
                && (r - 1..=r + 1).all(|row| !map[row as usize][c as usize])
        } else {
            (0 <= r && r < n && 1 <= c && c < n - 1)
                && (c - 1..=c + 1).all(|col| !map[r as usize][col as usize])
        }
    };

    let mut queue = VecDeque::from([(log_center, is_log_vertical, 0)]);

    while let Some(((r, c), is_vertical, count)) = queue.pop_front() {
        let next_count = count + 1;

        let moved = [(r - 1, c), (r, c - 1), (r + 1, c), (r, c + 1)].map(|(moved_r, moved_c)| {
            is_placeable(moved_r, moved_c, is_vertical).then_some(((moved_r, moved_c), is_vertical))
        });
        let turned = (c - 1..=c + 1)
            .all(|col| is_placeable(r, col, true))
            .then_some(((r, c), !is_vertical));

        for ((adj_r, adj_c), adj_is_vertical) in
            moved.into_iter().chain(std::iter::once(turned)).flatten()
        {
            if ((adj_r, adj_c), adj_is_vertical) == end {
                return Some(next_count);
            }

            let adj = (adj_r as usize, adj_c as usize, adj_is_vertical as usize);

            if visited[adj.0][adj.1][adj.2] || !is_placeable(adj_r, adj_c, adj_is_vertical) {
                continue;
            }

            visited[adj.0][adj.1][adj.2] = true;
            queue.push_back(((adj_r, adj_c), adj_is_vertical, next_count));
        }
    }

    None
}
