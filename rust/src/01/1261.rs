use std::collections::VecDeque;
use std::io;

const WIDTH_MAX: usize = 100;
const HEIGHT_MAX: usize = 100;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();

    let [width, height] = [(); 2].map(|_| input.next().unwrap().parse::<usize>().unwrap());
    let mut map = [[0; WIDTH_MAX]; HEIGHT_MAX];

    for (r, row) in input.enumerate() {
        for (c, ch) in row.bytes().enumerate() {
            map[r][c] = ch;
        }
    }

    let mut visited = [[false; WIDTH_MAX]; HEIGHT_MAX];
    visited[0][0] = true;

    let mut min_count = i32::MAX;
    let mut queue = VecDeque::from([((0, 0), 0)]);

    while let Some(((r, c), break_count)) = queue.pop_front() {
        if (r, c) == (height - 1, width - 1) {
            min_count = break_count.min(min_count);
            continue;
        }

        let adjacents = [
            (r.saturating_sub(1), c),
            (r, c.saturating_sub(1)),
            ((r + 1).min(height - 1), c),
            (r, (c + 1).min(width - 1)),
        ];

        for (adj_r, adj_c) in adjacents {
            if visited[adj_r][adj_c] {
                continue;
            }

            visited[adj_r][adj_c] = true;

            if map[adj_r][adj_c] == b'1' {
                queue.push_back(((adj_r, adj_c), break_count + 1));
            } else {
                queue.push_front(((adj_r, adj_c), break_count));
            }
        }
    }

    println!("{min_count}");
}
