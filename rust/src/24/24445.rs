use std::collections::VecDeque;
use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let mut input = || input.next().unwrap();
    let mut output = String::new();

    let (n, m, r) = (input(), input(), input());
    let mut adjacency_list = vec![Vec::new(); n + 1];

    for (u, v) in (0..m).map(|_| (input(), input())) {
        adjacency_list[u].push(v as i32);
        adjacency_list[v].push(u as i32);
    }

    for list in adjacency_list.iter_mut() {
        (*list).sort_unstable();
    }

    let visited = bfs(&adjacency_list, r);

    for order in &visited[1..] {
        writeln!(output, "{order}").unwrap();
    }

    print!("{output}");
}

fn bfs(graph: &[Vec<i32>], start: usize) -> Vec<i32> {
    let mut visited = vec![0; graph.len()];
    let mut queue = VecDeque::from([start as i32]);
    let mut count = 1;

    while let Some(node) = queue.pop_front() {
        if visited[node as usize] != 0 {
            continue;
        }

        visited[node as usize] = count;
        count += 1;

        for &adj in graph[node as usize].iter().rev() {
            queue.push_back(adj);
        }
    }

    visited
}
