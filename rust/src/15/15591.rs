use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let mut input = || input.next().unwrap();
    let mut output = String::new();

    let (n, q) = (input(), input());
    let mut adjacency_list = vec![Vec::new(); n + 1];

    for [a, b, c] in (0..n - 1).map(|_| [(); 3].map(|_| input())) {
        adjacency_list[a].push((b, c as i32));
        adjacency_list[b].push((a, c as i32));
    }

    for (k, v) in (0..q).map(|_| (input() as i32, input())) {
        writeln!(output, "{}", dfs(&adjacency_list, k, v)).unwrap();
    }

    print!("{output}");
}

fn dfs(graph: &[Vec<(usize, i32)>], k: i32, start: usize) -> i32 {
    let mut count = 0;
    let mut visited = vec![false; graph.len()];
    visited[start] = true;

    let mut stack = vec![(start, i32::MAX)];

    while let Some((node, dist)) = stack.pop() {
        for &(adj, adj_dist) in &graph[node] {
            if visited[adj] || adj_dist < k {
                continue;
            }

            visited[adj] = true;
            count += 1;
            stack.push((adj, dist.min(adj_dist)));
        }
    }

    count
}
