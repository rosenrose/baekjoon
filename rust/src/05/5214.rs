use std::collections::VecDeque;
use std::io;

const NODES_MAX: usize = 100_000 + 1 + 1000;
const EDGES_MAX: usize = 1000 * 2 * 1000;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let [n, k, m] = [(); 3].map(|_| input.next().unwrap());
    let mut adjacency_array = ([i32::MAX; NODES_MAX], [(0, 0); EDGES_MAX]);
    let mut i = 0;

    for tube in (0..m).map(|t| t + n + 1) {
        for station in input.by_ref().take(k) {
            let prev = adjacency_array.0[station];
            adjacency_array.0[station] = i as i32;
            adjacency_array.1[i] = (tube as i32, prev);
            i += 1;

            let prev = adjacency_array.0[tube];
            adjacency_array.0[tube] = i as i32;
            adjacency_array.1[i] = (station as i32, prev);
            i += 1;
        }
    }

    let min_count = bfs(&adjacency_array, 1, n as i32);

    println!("{}", if min_count == i32::MAX { -1 } else { min_count });
}

fn bfs((nodes, edges): &([i32; NODES_MAX], [(i32, i32); EDGES_MAX]), start: i32, end: i32) -> i32 {
    let mut min_count = i32::MAX;
    let is_tube = |node: i32| node > end;

    let mut visited = [false; NODES_MAX];
    visited[start as usize] = true;

    let mut queue = VecDeque::from([(start, 1)]);

    while let Some((node, count)) = queue.pop_front() {
        if node == end {
            min_count = count.min(min_count);
            continue;
        }

        let next_count = count + is_tube(node) as i32;
        let mut edge = nodes[node as usize];

        while let Some(&(adj, next_edge)) = edges.get(edge as usize) {
            if !visited[adj as usize] {
                visited[adj as usize] = true;
                queue.push_back((adj, next_count));
            }

            edge = next_edge;
        }
    }

    min_count
}
