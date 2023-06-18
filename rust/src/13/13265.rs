use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let mut input = || input.next().unwrap();

    for _ in 0..input() {
        let (n, m) = (input(), input());
        let mut adjacency_array = (vec![i32::MAX; n + 1], vec![(0, 0); m << 1]);

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
