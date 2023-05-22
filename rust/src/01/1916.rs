use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let mut input = || input.next().unwrap();

    let (n, m) = (input(), input());
    let mut adjacency_list = vec![Vec::new(); n + 1];

    for (u, v, w) in (0..m).map(|_| (input(), input(), input() as i32)) {
        adjacency_list[u].push((v as i32, w));
    }

    let (start, end) = (input(), input());
    let distances = dijkstra(&adjacency_list, start);

    println!("{}", distances[end]);
}

fn dijkstra(graph: &[Vec<(i32, i32)>], start: usize) -> Vec<i32> {
    let mut distances = vec![i32::MAX; graph.len()];
    distances[start] = 0;

    let mut queue = BinaryHeap::from([Reverse((0, start as i32))]);

    while let Some(Reverse((dist, node))) = queue.pop() {
        let min_dist = distances[node as usize];

        if dist > min_dist {
            continue;
        }

        for &(adj, weight) in graph[node as usize].iter() {
            let adj_min_dist = distances[adj as usize];
            let new_dist = min_dist + weight;

            if new_dist >= adj_min_dist {
                continue;
            }

            distances[adj as usize] = new_dist;
            queue.push(Reverse((new_dist, adj)));
        }
    }

    distances
}
