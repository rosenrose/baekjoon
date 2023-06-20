use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let [n, m, r] = [(); 3].map(|_| input.next().unwrap());
    let mut adjacency_list = vec![Vec::new(); n + 1];

    for [u, v] in (0..m).map(|_| [(); 2].map(|_| input.next().unwrap())) {
        adjacency_list[u].push(v as i32);
        adjacency_list[v].push(u as i32);
    }

    for list in &mut adjacency_list {
        (*list).sort_unstable();
    }

    let visited = dfs(&adjacency_list, r);
    let sum: i64 = visited[1..]
        .iter()
        .map(|&(depth, order)| depth as i64 * order as i64)
        .sum();

    println!("{sum}");
}

fn dfs(graph: &[Vec<i32>], start: usize) -> Vec<(i32, i32)> {
    let mut visited = vec![(-1, 0); graph.len()];
    let mut stack = vec![(start as i32, 0)];
    let mut count = 1;

    while let Some((node, depth)) = stack.pop() {
        if visited[node as usize].0 != -1 {
            continue;
        }

        visited[node as usize] = (depth, count);
        count += 1;

        for &adj in &graph[node as usize] {
            stack.push((adj, depth + 1));
        }
    }

    visited
}
