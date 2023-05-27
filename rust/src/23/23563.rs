use std::collections::VecDeque;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let input = buf.split_ascii_whitespace();

    let (mut start, mut end) = ((0, 0), (0, 0));
    let mut map: Vec<Vec<_>> = input
        .skip(2)
        .enumerate()
        .map(|(r, row)| {
            row.char_indices()
                .map(|(c, ch)| match ch {
                    '#' => None,
                    '.' => Some(i32::MAX),
                    'S' => {
                        start = (r, c);
                        Some(0)
                    }
                    'E' => {
                        end = (r, c);
                        Some(i32::MAX)
                    }
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();

    let mut queue = VecDeque::from([(start, 0)]);

    while let Some(((r, c), time)) = queue.pop_front() {
        let next_time = time + 1;

        for (adj_r, adj_c) in get_adjacents(r, c) {
            let Some(adj_time) = map[adj_r][adj_c] else {
                continue;
            };

            if is_adjacent_to_wall(r, c, &map) && is_adjacent_to_wall(adj_r, adj_c, &map) {
                if (adj_r, adj_c) == end {
                    println!("{time}");
                    return;
                }

                if adj_time <= time {
                    continue;
                }

                map[adj_r][adj_c] = Some(time);
                queue.push_front(((adj_r, adj_c), time));
            } else {
                if (adj_r, adj_c) == end {
                    println!("{next_time}");
                    return;
                }

                if adj_time <= next_time {
                    continue;
                }

                map[adj_r][adj_c] = Some(next_time);
                queue.push_back(((adj_r, adj_c), next_time));
            }
        }
    }
}

fn get_adjacents(r: usize, c: usize) -> [(usize, usize); 4] {
    [(r - 1, c), (r, c - 1), (r + 1, c), (r, c + 1)]
}

fn is_adjacent_to_wall(r: usize, c: usize, map: &[Vec<Option<i32>>]) -> bool {
    get_adjacents(r, c)
        .iter()
        .any(|&(adj_r, adj_c)| map[adj_r][adj_c].is_none())
}
