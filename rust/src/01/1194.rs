use std::collections::VecDeque;
use std::io;

#[derive(Debug)]
enum Cells {
    Empty,
    Wall,
    Key(u8),
    Door(u8),
    Exit,
}

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let input = buf.split_ascii_whitespace();

    let mut start = (0, 0);
    let map: Vec<Vec<_>> = input
        .skip(2)
        .enumerate()
        .map(|(r, row)| {
            row.as_bytes()
                .iter()
                .enumerate()
                .map(|(c, ch)| match ch {
                    b'.' => Cells::Empty,
                    b'#' => Cells::Wall,
                    b'a'..=b'f' => Cells::Key(ch - b'a'),
                    b'A'..=b'F' => Cells::Door(ch - b'A'),
                    b'0' => {
                        start = (r, c);
                        Cells::Empty
                    }
                    b'1' => Cells::Exit,
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();

    let step = simulate(map, start);

    println!("{step}");
}

fn simulate(mut map: Vec<Vec<Cells>>, start: (usize, usize)) -> i32 {
    let (width, height) = (map[0].len(), map.len());
    let mut visited = vec![vec![[false; 1 << 6]; width]; height];
    visited[start.0][start.1][0] = true;

    let mut queue = VecDeque::from([(start, 0, 0, Vec::<((usize, usize), u8)>::new())]);

    while let Some(((r, c), step, keys, opens)) = queue.pop_front() {
        for (open, _) in &opens {
            map[open.0][open.1] = Cells::Empty;
        }

        let next_step = step + 1;
        let adjacents = [
            (r.saturating_sub(1), c),
            (r, c.saturating_sub(1)),
            ((r + 1).min(height - 1), c),
            (r, (c + 1).min(width - 1)),
        ];

        for (adj_r, adj_c) in adjacents {
            let mut next_keys = keys;
            let mut next_opens = opens.clone();

            match map[adj_r][adj_c] {
                Cells::Exit => return next_step,
                Cells::Wall => continue,
                Cells::Key(key) => next_keys |= 1 << key,
                Cells::Door(door) => {
                    if (next_keys & (1 << door)) >> door == 1 {
                        next_opens.push(((adj_r, adj_c), door));
                    } else {
                        continue;
                    }
                }
                _ => (),
            }

            if visited[adj_r][adj_c][next_keys] {
                continue;
            }

            visited[adj_r][adj_c][next_keys] = true;
            queue.push_back(((adj_r, adj_c), next_step, next_keys, next_opens));
        }

        for (open, door) in opens {
            map[open.0][open.1] = Cells::Door(door);
        }
    }

    -1
}
