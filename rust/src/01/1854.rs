use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let mut input = || input.next().unwrap();

    let (n, m, k) = (input(), input(), input());
    let mut adjacency_list = vec![Vec::new(); n + 1];

    for (a, b, c) in (0..m).map(|_| (input(), input(), input() as i32)) {
        adjacency_list[a].push((b as i32, c));
    }

    let distances = dijkstra_nth(&adjacency_list, 1, k);
    // println!("{distances:?}");
    for dists in &distances[1..] {
        println!(
            "{}",
            if dists.len() < k {
                -1
            } else {
                *dists.peek().unwrap()
            }
        );
    }
}

fn dijkstra_nth(graph: &[Vec<(i32, i32)>], start: usize, order: usize) -> Vec<BinaryHeap<i32>> {
    let mut distances = vec![BinaryHeap::new(); graph.len()];
    distances[start].push(0);

    let mut queue = BinaryHeap::from([Reverse((0, start as i32))]);

    while let Some(Reverse((dist, node))) = queue.pop() {
        for &(neighbor, weight) in graph[node as usize].iter() {
            let new_dist = dist + weight;
            let neighbor_idx = neighbor as usize;

            if distances[neighbor_idx].len() < order {
                distances[neighbor_idx].push(new_dist);
                queue.push(Reverse((new_dist, neighbor)));

                continue;
            }

            if new_dist < *distances[neighbor_idx].peek().unwrap() {
                distances[neighbor_idx].pop();
                distances[neighbor_idx].push(new_dist);
                queue.push(Reverse((new_dist, neighbor)));
            }
        }
    }

    distances
}
