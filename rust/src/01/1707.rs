use std::io;

const NODES_MAX: usize = 20_000 + 1;
const EDGES_MAX: usize = 200_000 * 2;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let mut input = || input.next().unwrap();

    for _ in 0..input() {
        let (v, e) = (input(), input());
        let mut adjacency_array = ([i32::MAX; NODES_MAX], [(0, 0); EDGES_MAX]);

        for (i, a, b) in (0..e).map(|i| (i << 1, input(), input())) {
            let prev = adjacency_array.0[a];
            adjacency_array.0[a] = i as i32;
            adjacency_array.1[i] = (b as i32, prev);

            let prev = adjacency_array.0[b];
            adjacency_array.0[b] = (i + 1) as i32;
            adjacency_array.1[i + 1] = (a as i32, prev);
        }

        let is_bipartite = dfs(&adjacency_array, v + 1);

        println!("{}", if is_bipartite { "YES" } else { "NO" });
    }
}

fn dfs((nodes, edges): &([i32; NODES_MAX], [(i32, i32); EDGES_MAX]), nodes_len: usize) -> bool {
    let mut visited = [None; NODES_MAX];

    for start in 1..nodes_len {
        if visited[start].is_some() {
            continue;
        }

        visited[start] = Some(true);
        let mut stack = vec![start as i32];

        while let Some(node) = stack.pop() {
            let color = visited[node as usize].unwrap();
            let mut edge = nodes[node as usize];

            while let Some(&(adj, next_edge)) = edges.get(edge as usize) {
                if let Some(adj_color) = visited[adj as usize] {
                    if adj_color == color {
                        return false;
                    }
                } else {
                    visited[adj as usize] = Some(!color);
                    stack.push(adj);
                }

                edge = next_edge;
            }
        }
    }

    true
}
