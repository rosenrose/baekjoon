use std::collections::{HashSet, VecDeque};
use std::io;

const SIZE: usize = 3;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i8>);
    let mut map = [[0; SIZE]; SIZE];

    for cell in map.iter_mut().flatten() {
        *cell = input.next().unwrap();
    }

    let ordered = [[1, 2, 3], [4, 5, 6], [7, 8, 0]];
    let mut visited = HashSet::from([map]);
    let mut queue = VecDeque::from([(map, 0)]);

    while let Some((puzzle, step)) = queue.pop_front() {
        if puzzle == ordered {
            println!("{step}");
            return;
        }

        let (r, c) = get_empty_cell(&puzzle);
        let adjacents = [
            (r.saturating_sub(1), c),
            (r, c.saturating_sub(1)),
            ((r + 1).min(SIZE - 1), c),
            (r, (c + 1).min(SIZE - 1)),
        ];

        for (adj_r, adj_c) in adjacents {
            let mut next_puzzle = puzzle;

            (next_puzzle[r][c], next_puzzle[adj_r][adj_c]) =
                (next_puzzle[adj_r][adj_c], next_puzzle[r][c]);

            if visited.contains(&next_puzzle) {
                continue;
            }

            visited.insert(next_puzzle);
            queue.push_back((next_puzzle, step + 1));
        }
    }

    println!("-1");
}

fn get_empty_cell(map: &[[i8; SIZE]]) -> (usize, usize) {
    for (r, row) in map.iter().enumerate() {
        for (c, &cell) in row.iter().enumerate() {
            if cell == 0 {
                return (r, c);
            }
        }
    }

    unreachable!()
}
