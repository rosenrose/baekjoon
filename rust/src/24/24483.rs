use std::io;

const MAX: usize = 100_000 + 1;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let [n, m, r] = [(); 3].map(|_| input.next().unwrap());
    let mut adjacency_list = [(); MAX].map(|_| Vec::new());

    for [u, v] in (0..m).map(|_| [(); 2].map(|_| input.next().unwrap())) {
        adjacency_list[u].push(v as i32);
        adjacency_list[v].push(u as i32);
    }

    for list in &mut adjacency_list[..=n] {
        (*list).sort_unstable();
    }

    let visited = dfs(&adjacency_list[..=n], r);
    let sum: i64 = visited[1..=n]
        .iter()
        .map(|&(depth, order)| depth as i64 * order as i64)
        .sum();

    println!("{sum}");
}

fn dfs(graph: &[Vec<i32>], start: usize) -> [(i32, i32); MAX] {
    let mut visited = [(-1, 0); MAX];
    let mut stack = vec![(start as i32, 0)];
    let mut count = 1;

    while let Some((node, depth)) = stack.pop() {
        if visited[node as usize].0 != -1 {
            continue;
        }

        visited[node as usize] = (depth, count);
        count += 1;

        for &adj in graph[node as usize].iter().rev() {
            stack.push((adj, depth + 1));
        }
    }

    visited
}
