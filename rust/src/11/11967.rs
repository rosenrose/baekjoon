use std::collections::VecDeque;
use std::io;

const MAX: usize = 100;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let [n, m] = [(); 2].map(|_| input.next().unwrap());
    let start = (0, 0);
    let mut map = [[false; MAX]; MAX];
    map[start.0][start.1] = true;

    let mut switches = [(); MAX].map(|_| [(); MAX].map(|_| Vec::new()));

    for [y, x, a, b] in (0..m).map(|_| [(); 4].map(|_| input.next().unwrap() - 1)) {
        switches[y][x].push((a, b));
    }

    let mut visited = [[false; MAX]; MAX];
    visited[start.0][start.1] = true;

    let mut discovered_darks = [[false; MAX]; MAX];
    let mut count = 1;
    let mut queue = VecDeque::from([start]);

    while let Some((r, c)) = queue.pop_front() {
        for &switch in &switches[r][c] {
            if !map[switch.0][switch.1] {
                map[switch.0][switch.1] = true;
                count += 1;
            }

            if discovered_darks[switch.0][switch.1] && !visited[switch.0][switch.1] {
                visited[switch.0][switch.1] = true;
                queue.push_back(switch);
            }
        }

        let adjacents = [
            (r.saturating_sub(1), c),
            (r, c.saturating_sub(1)),
            ((r + 1).min(n - 1), c),
            (r, (c + 1).min(n - 1)),
        ];

        for (adj_r, adj_c) in adjacents {
            if visited[adj_r][adj_c] {
                continue;
            }

            if map[adj_r][adj_c] {
                visited[adj_r][adj_c] = true;
                queue.push_back((adj_r, adj_c));
            } else {
                discovered_darks[adj_r][adj_c] = true;
            }
        }
    }

    println!("{count}");
}
