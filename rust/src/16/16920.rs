use std::collections::VecDeque;
use std::io;

#[derive(Copy, Clone, Debug, PartialEq)]
enum Cells {
    Empty,
    Wall,
    Castle(u8),
}

const WIDTH_MAX: usize = 1000;
const HEIGHT_MAX: usize = 1000;
const PLAYER_MAX: usize = 9;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();

    let [height, width, p] = [(); 3].map(|_| parse_int(input.next().unwrap()));
    let mut ranges = [0; PLAYER_MAX];

    for range in &mut ranges[..p] {
        *range = parse_int(input.next().unwrap());
    }

    let mut castles = [(); PLAYER_MAX].map(|_| Vec::new());
    let mut areas = [0; PLAYER_MAX];
    let mut map = [[Cells::Empty; WIDTH_MAX]; HEIGHT_MAX];

    for (r, row) in input.enumerate() {
        for (c, ch) in row.char_indices() {
            map[r][c] = match ch {
                '.' => Cells::Empty,
                '#' => Cells::Wall,
                '1'..='9' => {
                    let player = ch as u8 - b'1';

                    castles[player as usize].push((r as i16, c as i16));
                    areas[player as usize] += 1;

                    Cells::Castle(player)
                }
                _ => unreachable!(),
            };
        }
    }

    let mut temp = VecDeque::new();

    while castles.iter().flatten().count() > 0 {
        for (player, borders) in castles.iter_mut().enumerate() {
            temp.resize(borders.len(), Default::default());

            for (i, &(r, c)) in borders.iter().enumerate() {
                temp[i] = ((r, c), 0);
            }

            borders.clear();

            while let Some(((r, c), dist)) = temp.pop_front() {
                let new_dist = dist + 1;
                let adjacents = [
                    ((r - 1).max(0), c),
                    (r, (c - 1).max(0)),
                    ((r + 1).min(height as i16 - 1), c),
                    (r, (c + 1).min(width as i16 - 1)),
                ];

                for (adj_r, adj_c) in adjacents {
                    if map[adj_r as usize][adj_c as usize] != Cells::Empty {
                        continue;
                    }

                    map[adj_r as usize][adj_c as usize] = Cells::Castle(player as u8);
                    areas[player as usize] += 1;

                    if new_dist == ranges[player as usize] {
                        borders.push((adj_r, adj_c));
                    } else {
                        temp.push_back(((adj_r, adj_c), new_dist));
                    }
                }
            }
        }
        // for r in &map {
        //     println!("{r:?}");
        // }
    }

    for area in &areas[..p] {
        print!("{area} ");
    }
}

fn parse_int(buf: &str) -> usize {
    buf.parse().unwrap()
}
