use std::io;

const NODES_MAX: usize = 1000 + 1;
const EDGES_MAX: usize = 100_000 * 2;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let mut input = || input.next().unwrap();

    for _ in 0..input() {
        let (_n, m) = (input(), input());
        let mut adjacency_array = ([i32::MAX; NODES_MAX], [(0, 0); EDGES_MAX]);

        for (i, (x, y)) in (0..m).map(|i| (i << 1, (input(), input()))) {
            let prev = adjacency_array.0[x];
            adjacency_array.0[x] = i as i32;
            adjacency_array.1[i] = (y as i32, prev);

            let prev = adjacency_array.0[y];
            adjacency_array.0[y] = (i + 1) as i32;
            adjacency_array.1[i + 1] = (x as i32, prev);
        }

        let is_bipartite = dfs(&adjacency_array);

        println!(
            "{}",
            if is_bipartite {
                "possible"
            } else {
                "impossible"
            }
        );
    }
}

fn dfs((nodes, edges): &([i32; NODES_MAX], [(i32, i32); EDGES_MAX])) -> bool {
    let mut visited = [None; NODES_MAX];

    for start in 1..nodes.len() {
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
