use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<usize>().unwrap());
    let mut input = || input.next().unwrap();

    let (n, m, x) = (input(), input(), input());
    let mut adjacency_list = vec![Vec::new(); n + 1];
    let mut adjacency_list_reverse = vec![Vec::new(); n + 1];

    for (start, end, t) in (0..m).map(|_| (input(), input(), input() as i32)) {
        adjacency_list[start].push((end, t));
        adjacency_list_reverse[end].push((start, t));
    }

    let dists_reverse = dijkstra(&adjacency_list_reverse, x);
    let dists = dijkstra(&adjacency_list, x);

    let max_dist = dists_reverse
        .iter()
        .skip(1)
        .zip(dists.iter().skip(1))
        .map(|(a, b)| a + b)
        .max()
        .unwrap();

    println!("{max_dist}");
}

fn dijkstra(graph: &Vec<Vec<(usize, i32)>>, start: usize) -> Vec<i32> {
    let mut distances = vec![i32::MAX; graph.len()];
    distances[start] = 0;

    let mut queue = BinaryHeap::from([Reverse((0, start))]);

    while let Some(Reverse((dist, node))) = queue.pop() {
        let min_dist = distances[node];

        if dist > min_dist {
            continue;
        }

        for &(neighbor, weight) in graph[node].iter() {
            let neighbor_min_dist = distances[neighbor];
            let new_dist = min_dist + weight;

            if new_dist >= neighbor_min_dist {
                continue;
            }

            distances[neighbor] = new_dist;
            queue.push(Reverse((new_dist, neighbor)));
        }
    }

    distances
}
