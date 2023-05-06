use std::collections::VecDeque;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let (n, m) = (input.next().unwrap(), input.next().unwrap());
    let map: Vec<Vec<_>> = (0..n)
        .map(|_| input.by_ref().take(m).map(|num| num == 1).collect())
        .collect();

    let mut max_dist = 0;
    let mut queue = VecDeque::with_capacity(n * m);

    for y in 0..n {
        for x in 0..m {
            let mut visited = vec![vec![false; m]; n];
            visited[y][x] = true;

            queue.push_back(((y, x), 0));

            while let Some(((row, col), dist)) = queue.pop_front() {
                if map[row][col] {
                    max_dist = dist.max(max_dist);
                    break;
                }

                let (up, down, left, right) = (
                    row.saturating_sub(1),
                    (row + 1).min(n - 1),
                    col.saturating_sub(1),
                    (col + 1).min(m - 1),
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

                for &(adj_r, adj_c) in adjacents.iter().filter(|&&adj| adj != (row, col)) {
                    if visited[adj_r][adj_c] {
                        continue;
                    }

                    visited[adj_r][adj_c] = true;
                    queue.push_back(((adj_r, adj_c), dist + 1));
                }
            }

            queue.clear();
        }
    }

    println!("{max_dist}");
}
