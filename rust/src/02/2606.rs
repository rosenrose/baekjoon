use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let mut input = || input.next().unwrap();

    let (nodes, edges) = (input(), input());
    let mut adjacency_list = vec![Vec::new(); nodes + 1];

    for (u, v) in (0..edges).map(|_| (input(), input())) {
        adjacency_list[u].push(v);
        adjacency_list[v].push(u);
    }

    println!("{}", dfs(&adjacency_list, 1));
}

fn dfs(graph: &[Vec<usize>], start: usize) -> i32 {
    let mut visited = vec![false; graph.len()];
    let mut count = 0;
    let mut stack = vec![start];
    visited[start] = true;

    while let Some(node) = stack.pop() {
        count += 1;

        for &adj in graph[node].iter() {
            if visited[adj] {
                continue;
            }

            visited[adj] = true;
            stack.push(adj);
        }
    }

    count - 1
}
