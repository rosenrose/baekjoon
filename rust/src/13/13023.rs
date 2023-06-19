use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_whitespace().flat_map(str::parse::<usize>);

    let [n, m] = [(); 2].map(|_| input.next().unwrap());
    let mut adjacency_array = (vec![usize::MAX; n], Vec::with_capacity(m * 2));

    for [a, b] in (0..m).map(|_| [(); 2].map(|_| input.next().unwrap())) {
        let prev = adjacency_array.0[a];
        adjacency_array.0[a] = adjacency_array.1.len();
        adjacency_array.1.push((b, prev));

        let prev = adjacency_array.0[b];
        adjacency_array.0[b] = adjacency_array.1.len();
        adjacency_array.1.push((a, prev));
    }

    let mut visited = vec![false; n];

    println!(
        "{}",
        u8::from((0..n).any(|start| dfs(&adjacency_array, start, 0, &mut visited)))
    );
}

fn dfs(
    graph: &(Vec<usize>, Vec<(usize, usize)>),
    node: usize,
    depth: i32,
    visited: &mut Vec<bool>,
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
