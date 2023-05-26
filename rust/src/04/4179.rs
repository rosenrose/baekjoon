use std::collections::VecDeque;
use std::io;

#[derive(Copy, Clone, PartialEq)]
enum Cells {
    Empty,
    Wall,
    Human,
    Fire,
}

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();

    let (height, width) = (
        parse_int(input.next().unwrap()),
        parse_int(input.next().unwrap()),
    );

    let mut queue = VecDeque::new();
    let mut map: Vec<Vec<_>> = input
        .enumerate()
        .map(|(r, row)| {
            row.char_indices()
                .map(|(c, ch)| match ch {
                    '.' => Cells::Empty,
                    '#' => Cells::Wall,
                    'J' => {
                        queue.push_back(((r as i32, c as i32), 0));
                        Cells::Human
                    }
                    'F' => {
                        queue.push_front(((r as i32, c as i32), 0));
                        Cells::Fire
                    }
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();

    while let Some(((r, c), time)) = queue.pop_front() {
        let cell = map[r as usize][c as usize];
        let new_time = time + 1;
        let adjacents = [(r - 1, c), (r, c - 1), (r + 1, c), (r, c + 1)];

        for (adj_r, adj_c) in adjacents {
            if !(0 <= adj_r && adj_r < height && 0 <= adj_c && adj_c < width) {
                if cell == Cells::Human {
                    println!("{new_time}");
                    return;
                }

                continue;
            }

            if map[adj_r as usize][adj_c as usize] != Cells::Empty {
                continue;
            }

            map[adj_r as usize][adj_c as usize] = cell;
            queue.push_back(((adj_r, adj_c), new_time));
        }
    }

    println!("IMPOSSIBLE");
}

fn parse_int(buf: &str) -> i32 {
    buf.parse().unwrap()
}
