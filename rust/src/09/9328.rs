use std::collections::{HashSet, VecDeque};
use std::io;

#[derive(Clone, Debug)]
enum Cells {
    Empty,
    Wall,
    Key(u8),
    Door(u8),
    Document,
}

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();
    let mut input = || input.next().unwrap();

    for _ in 0..parse_int(input()) {
        let (height, width) = (parse_int(input()), parse_int(input()));
        let mut map = vec![vec![Cells::Empty; width + 2]; height + 2];

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

        println!("{}", simulate(map, keys));
    }
}

fn simulate(mut map: Vec<Vec<Cells>>, mut keys: i32) -> i32 {
    let (width, height) = (map[0].len(), map.len());
    let start = (0, 0);

    let mut count = 0;
    let mut visited = vec![vec![HashSet::new(); width]; height];
    visited[start.0][start.1].insert(keys);

    let mut visited_doors = vec![Vec::<(usize, usize)>::new(); 26];
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

                    while let Some(door) = visited_doors[key as usize].pop() {
                        visited[door.0][door.1].insert(keys);
                        queue.push_front(door);
                    }
                }
                Cells::Document => {
                    map[adj_r][adj_c] = Cells::Empty;
                    count += 1;
                }
                Cells::Door(door) => {
                    if keys & (1 << door) == 0 {
                        visited_doors[door as usize].push((r, c));
                        continue;
                    }
                }
                _ => (),
            }

            if visited[adj_r][adj_c].contains(&keys) {
                continue;
            }

            visited[adj_r][adj_c].insert(keys);
            queue.push_back((adj_r, adj_c));
        }
    }

    count
}

fn parse_int(buf: &str) -> usize {
    buf.parse().unwrap()
}
