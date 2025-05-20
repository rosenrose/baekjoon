use std::collections::VecDeque;
use std::io;

const WIDTH_MAX: usize = 50;
const HEIGHT_MAX: usize = 50;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();

    let [height, width] = [(); 2].map(|_| input.next().unwrap().parse::<usize>().unwrap());
    let mut map = [[0; WIDTH_MAX]; HEIGHT_MAX];

    for (r, row) in input.map(str::as_bytes).enumerate() {
        for (c, &num) in row.iter().enumerate() {
            map[r][c] = num;
        }
    }

    let mut max_time = 0;

    for y in 0..height {
        for x in 0..width {
            if map[y][x] == b'W' {
                continue;
            }

            let mut visited = [[false; WIDTH_MAX]; HEIGHT_MAX];
            visited[y][x] = true;

            let mut time = 0;
            let mut queue = VecDeque::from([((y, x), 0)]);

            while let Some(((r, c), t)) = queue.pop_front() {
                time = time.max(t);

                let adjacents = [
                    (r.saturating_sub(1), c),
                    (r, c.saturating_sub(1)),
                    ((r + 1).min(height - 1), c),
                    (r, (c + 1).min(width - 1)),
                ];

                for (adj_r, adj_c) in adjacents {
                    if visited[adj_r][adj_c] || map[adj_r][adj_c] == b'W' {
                        continue;
                    }

                    visited[adj_r][adj_c] = true;
                    queue.push_back(((adj_r, adj_c), t + 1));
                }
            }

            max_time = time.max(max_time);
        }
    }

    println!("{max_time}");
}
