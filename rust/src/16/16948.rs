use std::collections::VecDeque;
use std::io;

const MAX: usize = 200;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut input = || input.next().unwrap();

    let n = input();
    let (start, end) = ((input(), input()), (input(), input()));

    let mut visited = [[false; MAX]; MAX];
    visited[start.0 as usize][start.1 as usize] = true;

    let mut queue = VecDeque::from([(start, 0)]);

    while let Some(((r, c), step)) = queue.pop_front() {
        if (r, c) == end {
            println!("{step}");
            return;
        }

        let adjacents = [
            (r - 2, c - 1),
            (r - 2, c + 1),
            (r, c - 2),
            (r, c + 2),
            (r + 2, c - 1),
            (r + 2, c + 1),
        ];

        for &(adj_r, adj_c) in adjacents
            .iter()
            .filter(|&&(adj_r, adj_c)| 0 <= adj_r && adj_r < n && 0 <= adj_c && adj_c < n)
        {
            if visited[adj_r as usize][adj_c as usize] {
                continue;
            }

            visited[adj_r as usize][adj_c as usize] = true;
            queue.push_back(((adj_r, adj_c), step + 1));
        }
    }

    println!("-1");
}
