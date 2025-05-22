use std::collections::VecDeque;
use std::io;

#[derive(Copy, Clone, Debug)]
enum Cells {
    Empty,
    Wall,
    Key(u8),
    Door(u8),
    Document,
}

const WIDTH_MAX: usize = 100 + 2;
const HEIGHT_MAX: usize = 100 + 2;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();
    let mut input = || input.next().unwrap();

    for _ in 0..parse_int(input()) {
        let [height, width] = [(); 2].map(|_| parse_int(input()));
        let mut map = [[Cells::Empty; WIDTH_MAX]; HEIGHT_MAX];

        for r in 0..height {
            for (c, ch) in input().as_bytes().iter().enumerate() {
                map[r + 1][c + 1] = match ch {
                    b'.' => Cells::Empty,
                    b'*' => Cells::Wall,
                    b'$' => Cells::Document,
                    b'a'..=b'z' => Cells::Key(ch - b'a'),
                    b'A'..=b'Z' => Cells::Door(ch - b'A'),
                    _ => unreachable!(),
                };
            }
        }

        let mut keys = 0;

        for &ch in input().as_bytes() {
            if ch == b'0' {
                break;
            }

            keys |= 1 << (ch - b'a');
        }

        println!("{}", simulate(&mut map[..height + 2], width + 2, keys));
    }
}

fn simulate(map: &mut [[Cells; WIDTH_MAX]], width: usize, mut keys: i32) -> i32 {
    let height = map.len();
    let start = (0, 0);

    let mut count = 0;
    let mut visited = [[None; WIDTH_MAX]; HEIGHT_MAX];
    visited[start.0][start.1] = Some(keys);

    let mut discovered_doors = [(); 26].map(|_| Vec::<(usize, usize)>::new());
    let mut queue = VecDeque::from([start]);

    while let Some((r, c)) = queue.pop_front() {
        let adjacents = [
            (r.saturating_sub(1), c),
            (r, c.saturating_sub(1)),
            ((r + 1).min(height - 1), c),
            (r, (c + 1).min(width - 1)),
        ];

        for (adj_r, adj_c) in adjacents {
            match map[adj_r][adj_c] {
                Cells::Wall => continue,
                Cells::Key(key) => {
                    keys |= 1 << key;

                    while let Some(door) = discovered_doors[key as usize].pop() {
                        visited[door.0][door.1] = Some(keys);
                        queue.push_front(door);
                    }
                }
                Cells::Document => {
                    map[adj_r][adj_c] = Cells::Empty;
                    count += 1;
                }
                Cells::Door(door) => {
                    if keys & (1 << door) == 0 {
                        discovered_doors[door as usize].push((r, c));
                        continue;
                    }
                }
                _ => (),
            }

            if visited[adj_r][adj_c] == Some(keys) {
                continue;
            }

            visited[adj_r][adj_c] = Some(keys);
            queue.push_back((adj_r, adj_c));
        }
    }

    count
}

fn parse_int(buf: &str) -> usize {
    buf.parse().unwrap()
}
