use std::io;

const NODES_MAX: usize = 2000;
const EDGES_MAX: usize = 2000 * 2;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_whitespace().flat_map(str::parse::<usize>);

    let [n, m] = [(); 2].map(|_| input.next().unwrap());
    let mut adjacency_array = ([usize::MAX; NODES_MAX], [(0, 0); EDGES_MAX]);

    for (i, [a, b]) in (0..m).map(|i| (i << 1, [(); 2].map(|_| input.next().unwrap()))) {
        let prev = adjacency_array.0[a];
        adjacency_array.0[a] = i;
        adjacency_array.1[i] = (b, prev);

        let prev = adjacency_array.0[b];
        adjacency_array.0[b] = i + 1;
        adjacency_array.1[i + 1] = (a, prev);
    }

    let mut visited = [false; NODES_MAX];

    println!(
        "{}",
        (0..n).any(|start| dfs(&adjacency_array, start, 0, &mut visited[..n])) as u8
    );
}

fn dfs(
    graph: &([usize; NODES_MAX], [(usize, usize); EDGES_MAX]),
    node: usize,
    depth: i32,
    visited: &mut [bool],
) -> bool {
    if depth == 4 {
        return true;
    }

    visited[node] = true;
    let (nodes, edges) = graph;
    let mut edge = nodes[node];

    while let Some(&(adj, next_edge)) = edges.get(edge) {
        if !visited[adj] {
            let is_finished = dfs(graph, adj, depth + 1, visited);

            if is_finished {
                return is_finished;
            }
        }

        edge = next_edge;
    }

    visited[node] = false;

    false
}
