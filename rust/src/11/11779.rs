use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io;

const NODES_MAX: usize = 1000 + 1;
const EDGES_MAX: usize = 100_000;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let mut input = || input.next().unwrap();

    let (n, m) = (input(), input());
    let mut adjacency_array = ([i32::MAX; NODES_MAX], [((0, 0), 0); EDGES_MAX]);

    for (i, [u, v, w]) in (0..m).map(|_| [(); 3].map(|_| input())).enumerate() {
        let prev = adjacency_array.0[u];

        adjacency_array.0[u] = i as i32;
        adjacency_array.1[i] = ((v as i32, w as i32), prev);
    }

    let (start, mut end) = (input() as i32, input() as i32);
    let (distance, prevs) = dijkstra_with_path(&adjacency_array, start, end);

    println!("{distance}");

    let mut path = [0; NODES_MAX];
    path[0] = end;
    let mut path_len = 1;

    while end != start {
        end = prevs[end as usize];
        path[path_len] = end;
        path_len += 1;
    }

    println!("{path_len}");

    for p in path[..path_len].iter().rev() {
        print!("{p} ");
    }
}

fn dijkstra_with_path(
    (nodes, edges): &([i32; NODES_MAX], [((i32, i32), i32); EDGES_MAX]),
    start: i32,
    end: i32,
) -> (i32, [i32; NODES_MAX]) {
    let mut distances = [i32::MAX; NODES_MAX];
    distances[start as usize] = 0;

    let mut prevs = [0; NODES_MAX];
    let mut queue = BinaryHeap::from([(Reverse(0), start)]);

    while let Some((Reverse(dist), node)) = queue.pop() {
        let min_dist = distances[node as usize];

        if node == end {
            return (min_dist, prevs);
        }
        if dist > min_dist {
            continue;
        }

        let mut edge = nodes[node as usize];

        while let Some(&((adj, weight), next_edge)) = edges.get(edge as usize) {
            let adj_min_dist = distances[adj as usize];
            let new_dist = min_dist + weight;

            if new_dist < adj_min_dist {
                distances[adj as usize] = new_dist;
                prevs[adj as usize] = node;
                queue.push((Reverse(new_dist), adj));
            }

            edge = next_edge;
        }
    }

    (distances[end as usize], prevs)
}
