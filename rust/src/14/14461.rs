use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io;

const CYCLE: u32 = 3;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<u32>);

    let (n, cross_time) = (input.next().unwrap() as usize, input.next().unwrap());
    let map: Vec<Vec<_>> = (0..n).map(|_| input.by_ref().take(n).collect()).collect();
    let distances = dijkstra(&map, (0, 0), cross_time);

    println!("{}", distances[n - 1][n - 1]);
}

fn dijkstra(graph: &[Vec<u32>], start: (i32, i32), cross_time: u32) -> Vec<Vec<u32>> {
    let n = graph.len();
    let mut distances = vec![vec![u32::MAX; n]; n];
    distances[start.0 as usize][start.1 as usize] = 0;

    let mut queue = BinaryHeap::from([Reverse((0, start))]);

    while let Some(Reverse((dist, (r, c)))) = queue.pop() {
        let min_dist = distances[r as usize][c as usize];

        if dist > min_dist {
            continue;
        }

        let dist_to_end = r.abs_diff(n as i32 - 1) + c.abs_diff(n as i32 - 1);

        if dist_to_end <= 2 {
            distances[n - 1][n - 1] = distances[n - 1][n - 1].min(dist + cross_time * dist_to_end);
        }

        let adjacents = [
            (r - 3, c),
            (r - 2, c - 1),
            (r - 2, c + 1),
            (r - 1, c - 2),
            (r - 1, c),
            (r - 1, c + 2),
            (r, c - 3),
            (r, c - 1),
            (r, c + 1),
            (r, c + 3),
            (r + 1, c - 2),
            (r + 1, c),
            (r + 1, c + 2),
            (r + 2, c - 1),
            (r + 2, c + 1),
            (r + 3, c),
        ]
        .into_iter()
        .filter(|&(adj_r, adj_c)| 0 <= adj_r && adj_r < n as i32 && 0 <= adj_c && adj_c < n as i32);

        for (adj_r, adj_c) in adjacents {
            let adj = (adj_r as usize, adj_c as usize);
            let eat_time = graph[adj.0][adj.1];
            let new_dist = min_dist + eat_time + (cross_time * CYCLE);
            let adj_min_dist = distances[adj.0][adj.1];

            if new_dist >= adj_min_dist {
                continue;
            }

            distances[adj.0][adj.1] = new_dist;
            queue.push(Reverse((new_dist, (adj_r, adj_c))));
        }
    }

    distances
}
