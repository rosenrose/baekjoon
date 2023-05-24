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

    let mut visited = vec![vec![false; width]; height];
    let mut queue = VecDeque::from([((0_usize, 0_usize), 1)]);
    visited[0][0] = true;

    while let Some(((r, c), step)) = queue.pop_front() {
        if (r, c) == (height - 1, width - 1) {
            println!("{step}");
            break;
        }

        let adjacents = [
            (r.saturating_sub(1), c),
            (r, c.saturating_sub(1)),
            ((r + 1).min(height - 1), c),
            (r, (c + 1).min(width - 1)),
        ];

        for &(adj_r, adj_c) in adjacents.iter().filter(|&&adj| adj != (r, c)) {
            if visited[adj_r][adj_c] || map[adj_r][adj_c] == b'0' {
                continue;
            }

            visited[adj_r][adj_c] = true;
            queue.push_back(((adj_r, adj_c), step + 1));
        }
    }
}

fn parse_int(buf: &str) -> usize {
    buf.parse().unwrap()
}
