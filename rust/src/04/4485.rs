use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    for i in 1.. {
        let n @ 1.. = input.next().unwrap() as usize else {
            return;
        };

        let cave: Vec<Vec<_>> = (0..n).map(|_| input.by_ref().take(n).collect()).collect();
        let distances = dijkstra(&cave, (0, 0));
        // println!("{distances:?}");
        println!("Problem {i}: {}", distances[n - 1][n - 1]);
    }
}

fn dijkstra(graph: &[Vec<i32>], start: (usize, usize)) -> Vec<Vec<i32>> {
    let n = graph.len();
    let mut distances = vec![vec![i32::MAX; n]; n];
    distances[start.0][start.1] = graph[start.0][start.1];

    let mut queue = BinaryHeap::from([Reverse((0, start))]);

    while let Some(Reverse((dist, (r, c)))) = queue.pop() {
        let min_dist = distances[r][c];

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
            queue.push(Reverse((new_dist, (adj_r, adj_c))));
        }
    }

    distances
}
