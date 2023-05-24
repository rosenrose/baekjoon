use std::collections::VecDeque;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();

    let (height, width) = (
        parse_int(input.next().unwrap()),
        parse_int(input.next().unwrap()),
    );
    let map: Vec<_> = input.map(str::as_bytes).collect();
    let mut max_time = 0;

    for y in 0..height {
        for x in 0..width {
            if map[y][x] == b'W' {
                continue;
            }

            let mut visited = vec![vec![false; width]; height];
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

                for &(adj_r, adj_c) in adjacents.iter().filter(|&&adj| adj != (r, c)) {
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

fn parse_int(buf: &str) -> usize {
    buf.parse().unwrap()
}
