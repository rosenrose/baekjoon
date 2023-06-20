use std::collections::VecDeque;
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

    let visited = bfs(&adjacency_list, r);
    let sum: i64 = visited[1..]
        .iter()
        .map(|&(depth, order)| depth as i64 * order as i64)
        .sum();

    println!("{sum}");
}

fn bfs(graph: &[Vec<i32>], start: usize) -> Vec<(i32, i32)> {
    let mut visited = vec![(-1, 0); graph.len()];
    visited[start] = (0, 1);

    let mut count = 2;
    let mut queue = VecDeque::from([(start as i32, 0)]);

    while let Some((node, depth)) = queue.pop_front() {
        let next_depth = depth + 1;

        for &adj in &graph[node as usize] {
            if visited[adj as usize].0 != -1 {
                continue;
            }

            visited[adj as usize] = (next_depth, count);
            count += 1;
            queue.push_back((adj, next_depth));
        }
    }

    visited
}
