use std::collections::VecDeque;
use std::io;

const WIDTH_MAX: usize = 100;
const HEIGHT_MAX: usize = 100;

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

    let start = (0, 0);

    let mut visited = [[false; WIDTH_MAX]; HEIGHT_MAX];
    visited[start.0][start.1] = true;

    let mut queue = VecDeque::from([(start, 1)]);

    while let Some(((r, c), step)) = queue.pop_front() {
        let next_step = step + 1;
        let adjacents = [
            (r.saturating_sub(1), c),
            (r, c.saturating_sub(1)),
            ((r + 1).min(height - 1), c),
            (r, (c + 1).min(width - 1)),
        ];

        for (adj_r, adj_c) in adjacents {
            if (adj_r, adj_c) == (height - 1, width - 1) {
                println!("{next_step}");
                return;
            }

            if visited[adj_r][adj_c] || map[adj_r][adj_c] == b'0' {
                continue;
            }

            visited[adj_r][adj_c] = true;
            queue.push_back(((adj_r, adj_c), next_step));
        }
    }
}
