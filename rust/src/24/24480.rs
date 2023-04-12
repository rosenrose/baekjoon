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
        adjacency_list[u].push(v);
        adjacency_list[v].push(u);
    }

    for list in adjacency_list.iter_mut() {
        (*list).sort_unstable();
    }

    let visited = dfs(&adjacency_list, r);

    for order in visited.iter().skip(1) {
        writeln!(output, "{order}").unwrap();
    }

    print!("{output}");
}

fn dfs(graph: &Vec<Vec<usize>>, start: usize) -> Vec<usize> {
    let mut visited = vec![0; graph.len()];
    let mut stack = vec![start];
    let mut count = 1;

    while let Some(node) = stack.pop() {
        if visited[node] != 0 {
            continue;
        }

        visited[node] = count;
        count += 1;

        for &neighbor in graph[node].iter() {
            stack.push(neighbor);
        }
    }

    visited
}
