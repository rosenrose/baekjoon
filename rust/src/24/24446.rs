use std::collections::VecDeque;
use std::fmt::Write;
use std::io;

const NODES_MAX: usize = 100_000 + 1;
const EDGES_MAX: usize = 200_000 * 2;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let mut output = String::new();

    let [n, m, r] = [(); 3].map(|_| input.next().unwrap());
    let mut adjacency_array = ([i32::MAX; NODES_MAX], [(0, 0); EDGES_MAX]);

    for (i, [u, v]) in (0..m).map(|i| (i << 1, [(); 2].map(|_| input.next().unwrap()))) {
        let prev = adjacency_array.0[u];
        adjacency_array.0[u] = i as i32;
        adjacency_array.1[i] = (v as i32, prev);

        let prev = adjacency_array.0[v];
        adjacency_array.0[v] = (i + 1) as i32;
        adjacency_array.1[i + 1] = (u as i32, prev);
    }

    let visited = bfs(&adjacency_array, r);

    for depth in &visited[1..=n] {
        writeln!(output, "{depth}").unwrap();
    }

    print!("{output}");
}

fn bfs(
    (nodes, edges): &([i32; NODES_MAX], [(i32, i32); EDGES_MAX]),
    start: usize,
) -> [i32; NODES_MAX] {
    let mut visited = [-1; NODES_MAX];
    visited[start] = 0;

    let mut queue = VecDeque::from([(start as i32, 0)]);

    while let Some((node, depth)) = queue.pop_front() {
        let next_depth = depth + 1;
        let mut edge = nodes[node as usize];

        while let Some(&(adj, next_edge)) = edges.get(edge as usize) {
            if visited[adj as usize] == -1 {
                visited[adj as usize] = next_depth;
                queue.push_back((adj, next_depth));
            }

            edge = next_edge;
        }
    }

    visited
}
