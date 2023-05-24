use std::collections::VecDeque;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut input = || input.next().unwrap();

    'outer: for _ in 0..input() {
        let n = input();
        let start = (input(), input());
        let end = (input(), input());

        let mut visited = vec![vec![false; n as usize]; n as usize];
        visited[start.0 as usize][start.1 as usize] = true;

        let mut queue = VecDeque::from([((start.0, start.1), 0)]);

        while let Some(((r, c), step)) = queue.pop_front() {
            if (r, c) == end {
                println!("{step}");
                continue 'outer;
            }

            let adjacents = [
                (r - 1, c - 2),
                (r - 2, c - 1),
                (r - 2, c + 1),
                (r - 1, c + 2),
                (r + 1, c - 2),
                (r + 2, c - 1),
                (r + 2, c + 1),
                (r + 1, c + 2),
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
    }
}
