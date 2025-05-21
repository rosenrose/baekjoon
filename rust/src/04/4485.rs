use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io;

const MAX: usize = 125;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    for i in 1.. {
        let n @ 1.. = input.next().unwrap() as usize else {
            return;
        };

        let mut cave = [[0; MAX]; MAX];

        for r in 0..n {
            for (c, num) in input.by_ref().take(n).enumerate() {
                cave[r][c] = num;
            }
        }

        let distance = dijkstra(&cave[..n], (0, 0), (n - 1, n - 1));

        println!("Problem {i}: {distance}");
    }
}

fn dijkstra(graph: &[[i32; MAX]], start: (usize, usize), end: (usize, usize)) -> i32 {
    let n = graph.len();
    let mut distances = [[i32::MAX; MAX]; MAX];
    distances[start.0][start.1] = graph[start.0][start.1];

    let mut queue = BinaryHeap::from([(Reverse(0), start)]);

    while let Some((Reverse(dist), (r, c))) = queue.pop() {
        let min_dist = distances[r][c];

        if (r, c) == end {
            return min_dist;
        }
        if dist > min_dist {
            continue;
        }

        let adjacents = [
            (r.saturating_sub(1), c),
            (r, c.saturating_sub(1)),
            ((r + 1).min(n - 1), c),
            (r, (c + 1).min(n - 1)),
        ];

        for (adj_r, adj_c) in adjacents {
            let weight = graph[adj_r][adj_c];
            let new_dist = min_dist + weight;
            let adj_min_dist = distances[adj_r][adj_c];

            if new_dist >= adj_min_dist {
                continue;
            }

            distances[adj_r][adj_c] = new_dist;
            queue.push((Reverse(new_dist), (adj_r, adj_c)));
        }
    }

    distances[end.0][end.1]
}
