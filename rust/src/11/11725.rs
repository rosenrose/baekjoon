use std::fmt::Write;
use std::io;

const MAX: usize = 100_000;
const NODES_MAX: usize = MAX + 1;
const EDGES_MAX: usize = (MAX - 1) * 2;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let mut output = String::new();

    let n = input.next().unwrap();
    let mut adjacency_array = ([i32::MAX; NODES_MAX], [(0, 0); EDGES_MAX]);

    for (i, [a, b]) in (0..n - 1).map(|i| (i << 1, [(); 2].map(|_| input.next().unwrap()))) {
        let prev = adjacency_array.0[a];
        adjacency_array.0[a] = i as i32;
        adjacency_array.1[i] = (b as i32, prev);

        let prev = adjacency_array.0[b];
        adjacency_array.0[b] = (i + 1) as i32;
        adjacency_array.1[i + 1] = (a as i32, prev);
    }

    let parents = dfs(&adjacency_array, 1);

    for parent in &parents[2..=n] {
        writeln!(output, "{parent}").unwrap();
    }

    print!("{output}");
}

fn dfs(
    (nodes, edges): &([i32; NODES_MAX], [(i32, i32); EDGES_MAX]),
    start: i32,
) -> [i32; NODES_MAX] {
    let mut parents = [0; NODES_MAX];
    let mut stack = vec![start];

    while let Some(node) = stack.pop() {
        let mut edge = nodes[node as usize];

        while let Some(&(adj, next_edge)) = edges.get(edge as usize) {
            if parents[adj as usize] == 0 {
                parents[adj as usize] = node;
                stack.push(adj);
            }

            edge = next_edge;
        }
    }

    parents
}
