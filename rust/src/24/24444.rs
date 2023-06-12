use std::collections::VecDeque;
use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut input = || input.next().unwrap();
    let mut output = String::new();

    let [n, m, r] = [(); 3].map(|_| input());
    let mut adjacency_list = vec![Vec::new(); n as usize + 1];

    for (u, v) in (0..m).map(|_| (input(), input())) {
        adjacency_list[u as usize].push(v);
        adjacency_list[v as usize].push(u);
    }

    for list in &mut adjacency_list {
        (*list).sort_unstable();
    }

    let visited = bfs(&adjacency_list, r);

    for order in &visited[1..] {
        writeln!(output, "{order}").unwrap();
    }

    print!("{output}");
}

fn bfs(graph: &[Vec<i32>], start: i32) -> Vec<i32> {
    let mut visited = vec![0; graph.len()];
    let mut queue = VecDeque::from([start]);
    let mut count = 1;

    while let Some(node) = queue.pop_front() {
        if visited[node as usize] != 0 {
            continue;
        }

        visited[node as usize] = count;
        count += 1;

        for &adj in &graph[node as usize] {
            queue.push_back(adj);
        }
    }

    visited
}
