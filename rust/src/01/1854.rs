use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let [n, m, k] = [(); 3].map(|_| input.next().unwrap());
    let mut adjacency_list = vec![Vec::new(); n + 1];

    for [a, b, c] in (0..m).map(|_| [(); 3].map(|_| input.next().unwrap())) {
        adjacency_list[a].push((b as i32, c as i32));
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

    let mut queue = BinaryHeap::from([(Reverse(0), start as i32)]);

    while let Some((Reverse(dist), node)) = queue.pop() {
        for &(adj, weight) in &graph[node as usize] {
            let new_dist = dist + weight;
            let adj_idx = adj as usize;

            if distances[adj_idx].len() < order {
                distances[adj_idx].push(new_dist);
                queue.push((Reverse(new_dist), adj));

                continue;
            }

            if new_dist < *distances[adj_idx].peek().unwrap() {
                distances[adj_idx].pop();
                distances[adj_idx].push(new_dist);
                queue.push((Reverse(new_dist), adj));
            }
        }
    }

    distances
}
