use std::cmp::Ordering;
use std::io;

const DIRS: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let [height, width, k] = [(); 3].map(|_| input.next().unwrap());
    let map: Vec<Vec<_>> = (0..height)
        .map(|_| input.by_ref().take(width as usize).collect())
        .collect();

    let (mut r, mut c) = (0, 0);
    let mut dir = 0;
    let (mut top, mut north, mut east, mut west, mut south, mut bottom) = (1, 2, 3, 4, 5, 6);

    let get_moved_coord = |r: i32, c: i32, dir: usize| {
        (
            (r + DIRS[dir].0).clamp(0, height - 1),
            (c + DIRS[dir].1).clamp(0, width - 1),
        )
    };

    let sum: i32 = (0..k)
        .map(|_| {
            let moved = get_moved_coord(r, c, dir);

            (r, c) = if moved == (r, c) {
                dir = (dir + 2) % DIRS.len();
                get_moved_coord(r, c, dir)
            } else {
                moved
            };

            match dir {
                0 => (top, bottom, east, west) = (west, east, top, bottom),
                1 => (top, bottom, north, south) = (north, south, bottom, top),
                2 => (top, bottom, east, west) = (east, west, bottom, top),
                3 => (top, bottom, north, south) = (south, north, top, bottom),
                _ => unreachable!(),
            }

            let score = get_score(r, c, &map);

            dir = match bottom.cmp(&map[r as usize][c as usize]) {
                Ordering::Greater => (dir + 1) % DIRS.len(),
                Ordering::Less => (dir + (DIRS.len() - 1)) % DIRS.len(),
                _ => dir,
            };

            score
        })
        .sum();

    println!("{sum}");
}

fn get_score(y: i32, x: i32, map: &[Vec<i32>]) -> i32 {
    let (width, height) = (map[0].len(), map.len());
    let mut visited = vec![vec![false; width]; height];
    visited[y as usize][x as usize] = true;

    let num = map[y as usize][x as usize];
    let mut count = 0;
    let mut stack = vec![(y, x)];

    while let Some((r, c)) = stack.pop() {
        count += 1;

        let adjacents = [
            ((r - 1).max(0), c),
            (r, (c - 1).max(0)),
            ((r + 1).min(height as i32 - 1), c),
            (r, (c + 1).min(width as i32 - 1)),
        ];

        for (adj_r, adj_c) in adjacents {
            let adj = (adj_r as usize, adj_c as usize);

            if visited[adj.0][adj.1] || map[adj.0][adj.1] != num {
                continue;
            }

            visited[adj.0][adj.1] = true;
            stack.push((adj_r, adj_c));
        }
    }

    num * count
}
