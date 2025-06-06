use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io;

const MAX: usize = 100;
const CYCLE: i32 = 3;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let [n, cross_time] = [(); 2].map(|_| input.next().unwrap() as usize);
    let mut map = [[0; MAX]; MAX];

    for r in 0..n {
        for (c, num) in input.by_ref().take(n).enumerate() {
            map[r][c] = num;
        }
    }

    let distance = dijkstra(&map[..n], (0, 0), (n - 1, n - 1), cross_time as i32);

    println!("{distance}");
}

fn dijkstra(
    graph: &[[i32; MAX]],
    start: (usize, usize),
    end: (usize, usize),
    cross_time: i32,
) -> i32 {
    let n = graph.len() as i32;
    let mut distances = [[i32::MAX; MAX]; MAX];
    distances[start.0][start.1] = 0;

    let mut queue = BinaryHeap::from([(Reverse(0), (start.0 as i32, start.1 as i32))]);

    while let Some((Reverse(dist), (r, c))) = queue.pop() {
        let min_dist = distances[r as usize][c as usize];

        if (r as usize, c as usize) == end {
            return min_dist;
        }
        if dist > min_dist {
            continue;
        }

        let dist_to_end = (r.abs_diff(end.0 as i32) + c.abs_diff(end.1 as i32)) as i32;

        if dist_to_end <= 2 {
            distances[end.0][end.1] = distances[end.0][end.1].min(dist + cross_time * dist_to_end);
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
        .filter(|&(adj_r, adj_c)| 0 <= adj_r && adj_r < n && 0 <= adj_c && adj_c < n);

        for (adj_r, adj_c) in adjacents {
            let adj = (adj_r as usize, adj_c as usize);
            let eat_time = graph[adj.0][adj.1];
            let new_dist = min_dist + eat_time + (cross_time * CYCLE);
            let adj_min_dist = distances[adj.0][adj.1];

            if new_dist >= adj_min_dist {
                continue;
            }

            distances[adj.0][adj.1] = new_dist;
            queue.push((Reverse(new_dist), (adj_r, adj_c)));
        }
    }

    distances[end.0][end.1]
}
