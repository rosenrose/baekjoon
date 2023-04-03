use std::collections::VecDeque;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();

    let (n, m) = (
        parse_int(input.next().unwrap()),
        parse_int(input.next().unwrap()),
    );
    let mut maze: Vec<Vec<_>> = input
        .map(|row| row.chars().map(|ch| (ch, 1)).collect())
        .collect();

    let mut queue = VecDeque::from([((0_usize, 0_usize), 1)]);

    while let Some(((x, y), step)) = queue.pop_front() {
        let adjacents = [
            (x.saturating_sub(1), y),
            (x, y.saturating_sub(1)),
            ((x + 1).min(m - 1), y),
            (x, (y + 1).min(n - 1)),
        ];

        for &(adj_x, adj_y) in adjacents.iter().filter(|&&adj| adj != (x, y)) {
            if maze[adj_y][adj_x].0 == '0' || maze[adj_y][adj_x].1 != 1 {
                continue;
            }

            maze[adj_y][adj_x].1 = step + 1;
            queue.push_back(((adj_x, adj_y), step + 1));
        }
    }

    println!("{}", maze[n - 1][m - 1].1);
}

fn parse_int(buf: &str) -> usize {
    buf.parse().unwrap()
}
