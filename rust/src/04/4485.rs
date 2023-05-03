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

fn dijkstra(graph: &[Vec<i32>], (start_x, start_y): (usize, usize)) -> Vec<Vec<i32>> {
    let n = graph.len();
    let mut distances = vec![vec![i32::MAX; n]; n];
    distances[start_y][start_x] = graph[start_y][start_x];

    let mut queue = BinaryHeap::from([Reverse((0, (start_x, start_y)))]);

    while let Some(Reverse((dist, (x, y)))) = queue.pop() {
        let min_dist = distances[y][x];

        if dist > min_dist {
            continue;
        }

        let adjacents = [
            (x.saturating_sub(1), y),
            (x, y.saturating_sub(1)),
            ((x + 1).min(n - 1), y),
            (x, (y + 1).min(n - 1)),
        ];

        for &(adj_x, adj_y) in adjacents.iter().filter(|&&adj| adj != (x, y)) {
            let weight = graph[adj_y][adj_x];
            let new_dist = min_dist + weight;
            let adj_min_dist = distances[adj_y][adj_x];

            if new_dist >= adj_min_dist {
                continue;
            }

            distances[adj_y][adj_x] = new_dist;
            queue.push(Reverse((new_dist, (adj_x, adj_y))));
        }
    }

    distances
}
