use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let mut input = || input.next().unwrap();

    for _ in 0..input() {
        let (v, e) = (input(), input());
        let mut adjacency_array = (vec![i32::MAX; v + 1], vec![(0, 0); e << 1]);

        for (i, (a, b)) in (0..e).map(|i| (i << 1, (input(), input()))) {
            let prev = adjacency_array.0[a];
            adjacency_array.0[a] = i as i32;
            adjacency_array.1[i] = (b as i32, prev);

            let prev = adjacency_array.0[b];
            adjacency_array.0[b] = (i + 1) as i32;
            adjacency_array.1[i + 1] = (a as i32, prev);
        }

        let is_bipartite = dfs(&adjacency_array);

        println!("{}", if is_bipartite { "YES" } else { "NO" });
    }
}

fn dfs((nodes, edges): &(Vec<i32>, Vec<(i32, i32)>)) -> bool {
    let mut visited = vec![None; nodes.len()];

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
