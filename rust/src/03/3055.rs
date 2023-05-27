use std::collections::VecDeque;
use std::io;

#[derive(Copy, Clone, PartialEq, Debug)]
enum Cells {
    Empty,
    Water,
    Stone,
    Home,
    Animal,
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
                    '*' => {
                        queue.push_front(((r, c), 0));
                        Cells::Water
                    }
                    'X' => Cells::Stone,
                    'D' => Cells::Home,
                    'S' => {
                        queue.push_back(((r, c), 0));
                        Cells::Animal
                    }
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();

    while let Some(((r, c), time)) = queue.pop_front() {
        let cell = map[r][c];
        let next_time = time + 1;

        let adjacents = [
            (r.saturating_sub(1), c),
            (r, c.saturating_sub(1)),
            ((r + 1).min(height - 1), c),
            (r, (c + 1).min(width - 1)),
        ];

        for (adj_r, adj_c) in adjacents {
            let adj_cell = map[adj_r][adj_c];

            if (cell, adj_cell) == (Cells::Animal, Cells::Home) {
                println!("{next_time}");
                return;
            }

            if adj_cell != Cells::Empty {
                continue;
            }

            map[adj_r][adj_c] = cell;
            queue.push_back(((adj_r, adj_c), next_time));
        }
    }

    println!("KAKTUS");
}

fn parse_int(buf: &str) -> usize {
    buf.parse().unwrap()
}
