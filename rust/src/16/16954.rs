use std::collections::VecDeque;
use std::io;

const SIZE: usize = 8;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.lines().flat_map(|line| line.chars());
    let mut map = [[false; SIZE]; SIZE];

    for cell in map.iter_mut().flatten() {
        *cell = input.next().unwrap() == '#';
    }

    println!("{}", simulate(map) as u8);
}

fn simulate(map: [[bool; SIZE]; SIZE]) -> bool {
    let (start, end) = ((SIZE - 1, 0), (0, SIZE - 1));
    let mut visited = [[false; SIZE]; SIZE];
    visited[start.0][start.1] = true;

    let mut queue = VecDeque::from([(start, map)]);

    while let Some(((row, col), map)) = queue.pop_front() {
        if (row, col) == end {
            return true;
        }

        let moved_map = get_moved_map(map);

        let (up, down, left, right) = (
            row.saturating_sub(1),
            (row + 1).min(SIZE - 1),
            col.saturating_sub(1),
            (col + 1).min(SIZE - 1),
        );
        let adjacents = [
            (up, left),
            (up, col),
            (up, right),
            (row, left),
            (row, right),
            (down, left),
            (down, col),
            (down, right),
        ];

        for (adj_r, adj_c) in adjacents {
            if visited[adj_r][adj_c] || map[adj_r][adj_c] || moved_map[adj_r][adj_c] {
                continue;
            }

            visited[adj_r][adj_c] = true;
            queue.push_back(((adj_r, adj_c), moved_map));
        }

        if !moved_map[row][col] {
            queue.push_back(((row, col), moved_map));
        }
    }

    false
}

fn get_moved_map(mut map: [[bool; SIZE]; SIZE]) -> [[bool; SIZE]; SIZE] {
    for r in (1..SIZE).rev() {
        for c in 0..SIZE {
            map[r][c] = map[r - 1][c];
        }
    }

    for c in 0..SIZE {
        map[0][c] = false;
    }

    map
}
