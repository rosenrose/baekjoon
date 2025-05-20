use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::fmt::Write;
use std::io;

const NODES_MAX: usize = 20000 + 1;
const EDGES_MAX: usize = 300_000;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let mut output = String::new();

    let [v, e, k] = [(); 3].map(|_| input.next().unwrap());
    let mut adjacency_array = ([i32::MAX; NODES_MAX], [((0, 0), 0); EDGES_MAX]);

    for (i, [u, v, w]) in (0..e).map(|i| (i, [(); 3].map(|_| input.next().unwrap()))) {
        let prev = adjacency_array.0[u];

        adjacency_array.0[u] = i as i32;
        adjacency_array.1[i] = ((v as i32, w as i32), prev);
    }

    let distances = dijkstra(&adjacency_array, k);
    // println!("{distances:?}");
    for &dist in &distances[1..=v] {
        if dist == i32::MAX {
            writeln!(output, "INF")
        } else {
            writeln!(output, "{dist}")
        }
        .unwrap();
    }

    print!("{output}");
}

fn dijkstra(
    (nodes, edges): &([i32; NODES_MAX], [((i32, i32), i32); EDGES_MAX]),
    start: usize,
) -> [i32; NODES_MAX] {
    let mut distances = [i32::MAX; NODES_MAX];
    distances[start] = 0;

    let mut queue = BinaryHeap::from([(Reverse(0), start as i32)]);

    while let Some((Reverse(dist), node)) = queue.pop() {
        let min_dist = distances[node as usize];
        let mut edge = nodes[node as usize];

        if dist > min_dist {
            continue;
        }

        while let Some(&((adj, weight), next_edge)) = edges.get(edge as usize) {
            let adj_min_dist = distances[adj as usize];
            let new_dist = min_dist + weight;

            if new_dist < adj_min_dist {
                distances[adj as usize] = new_dist;
                queue.push((Reverse(new_dist), adj));
            }

            edge = next_edge;
        }
    }

    distances
}
