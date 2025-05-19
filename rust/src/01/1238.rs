use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io;

const MAX: usize = 1000 + 1;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let [n, m, x] = [(); 3].map(|_| input.next().unwrap());
    let mut adjacency_list = [(); MAX].map(|_| Vec::new());
    let mut adjacency_list_reverse = [(); MAX].map(|_| Vec::new());

    for [start, end, t] in (0..m).map(|_| [(); 3].map(|_| input.next().unwrap())) {
        adjacency_list[start].push((end, t as i32));
        adjacency_list_reverse[end].push((start, t as i32));
    }

    let dists_reverse = dijkstra(&adjacency_list_reverse, x);
    let dists = dijkstra(&adjacency_list, x);

    let max_dist = dists_reverse[1..=n]
        .iter()
        .zip(&dists[1..=n])
        .map(|(a, b)| a + b)
        .max()
        .unwrap();

    println!("{max_dist}");
}

fn dijkstra(graph: &[Vec<(usize, i32)>], start: usize) -> [i32; MAX] {
    let mut distances = [i32::MAX; MAX];
    distances[start] = 0;

    let mut queue = BinaryHeap::from([(Reverse(0), start)]);

    while let Some((Reverse(dist), node)) = queue.pop() {
        let min_dist = distances[node];

        if dist > min_dist {
            continue;
        }

        for &(adj, weight) in &graph[node] {
            let adj_min_dist = distances[adj];
            let new_dist = min_dist + weight;

            if new_dist >= adj_min_dist {
                continue;
            }

            distances[adj] = new_dist;
            queue.push((Reverse(new_dist), adj));
        }
    }

    distances
}
