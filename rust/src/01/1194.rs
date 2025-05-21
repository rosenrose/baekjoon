use std::collections::VecDeque;
use std::io;

#[derive(Copy, Clone)]
enum Cells {
    Empty,
    Wall,
    Key(u8),
    Door(u8),
    Exit,
}

const WIDTH_MAX: usize = 50;
const HEIGHT_MAX: usize = 50;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();

    let [height, width] = [(); 2].map(|_| input.next().unwrap().parse::<usize>().unwrap());
    let mut start = (0, 0);
    let mut map = [[Cells::Empty; WIDTH_MAX]; HEIGHT_MAX];

    for (r, row) in input.enumerate() {
        for (c, ch) in row.as_bytes().iter().enumerate() {
            map[r][c] = match ch {
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
            };
        }
    }

    let step = simulate(&map[..height], width, start);

    println!("{step}");
}

fn simulate(map: &[[Cells; WIDTH_MAX]], width: usize, start: (usize, usize)) -> i32 {
    let height = map.len();
    let mut visited = [[[false; 1 << 6]; WIDTH_MAX]; HEIGHT_MAX];
    visited[start.0][start.1][0] = true;

    let mut queue = VecDeque::from([(start, 0, 0)]);

    while let Some(((r, c), step, keys)) = queue.pop_front() {
        let next_step = step + 1;
        let adjacents = [
            (r.saturating_sub(1), c),
            (r, c.saturating_sub(1)),
            ((r + 1).min(height - 1), c),
            (r, (c + 1).min(width - 1)),
        ];

        for (adj_r, adj_c) in adjacents {
            match map[adj_r][adj_c] {
                Cells::Exit => return next_step,
                Cells::Wall => continue,
                Cells::Door(door) => {
                    if keys & (1 << door) == 0 {
                        continue;
                    }
                }
                _ => (),
            }

            if visited[adj_r][adj_c][keys] {
                continue;
            }

            visited[adj_r][adj_c][keys] = true;

            if let Cells::Key(key) = map[adj_r][adj_c] {
                let next_keys = keys | (1 << key);

                visited[adj_r][adj_c][next_keys] = true;
                queue.push_back(((adj_r, adj_c), next_step, next_keys));
            } else {
                queue.push_back(((adj_r, adj_c), next_step, keys));
            }
        }
    }

    -1
}
