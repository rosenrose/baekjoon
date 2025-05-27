use std::collections::VecDeque;
use std::fmt::Write;
use std::io;

const NODES_MAX: usize = 300_000 + 1;
const EDGES_MAX: usize = 1_000_000;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let mut output = String::new();

    let [_n, m, k, x] = [(); 4].map(|_| input.next().unwrap());
    let mut adjacency_array = ([i32::MAX; NODES_MAX], [(0, 0); EDGES_MAX]);

    for (i, [u, v]) in (0..m)
        .map(|_| [(); 2].map(|_| input.next().unwrap()))
        .enumerate()
    {
        let prev = adjacency_array.0[u];

        adjacency_array.0[u] = i as i32;
        adjacency_array.1[i] = (v as i32, prev);
    }

    let mut result = bfs(&adjacency_array, x, k as i32);

    if result.is_empty() {
        println!("-1");
        return;
    }

    result.sort_unstable();

    for node in result {
        writeln!(output, "{node}").unwrap();
    }

    print!("{output}");
}

fn bfs(
    (nodes, edges): &([i32; NODES_MAX], [(i32, i32); EDGES_MAX]),
    start: usize,
    target_dist: i32,
) -> Vec<i32> {
    let mut result = Vec::with_capacity(NODES_MAX >> 1);
    let mut visited = [false; NODES_MAX];
    visited[start] = true;

    let mut queue = VecDeque::from([(start as i32, 0)]);

    while let Some((node, dist)) = queue.pop_front() {
        if dist == target_dist {
            result.push(node);
            continue;
        }

        let mut edge = nodes[node as usize];

        while let Some(&(adj, next_edge)) = edges.get(edge as usize) {
            if !visited[adj as usize] {
                visited[adj as usize] = true;

                if dist < target_dist {
                    queue.push_back((adj, dist + 1));
                }
            }

            edge = next_edge;
        }
    }

    result
}
