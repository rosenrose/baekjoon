use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut input = || input.next().unwrap();
    let mut output = String::new();

    let n = input() as usize;
    let mut adjacency_list = vec![Vec::new(); n + 1];

    for (a, b) in (0..n - 1).map(|_| (input(), input())) {
        adjacency_list[a as usize].push(b);
        adjacency_list[b as usize].push(a);
    }

    let parents = dfs(&adjacency_list, 1);

    for parent in &parents[2..] {
        writeln!(output, "{parent}").unwrap();
    }

    print!("{output}");
}

fn dfs(graph: &[Vec<i32>], start: i32) -> Vec<i32> {
    let mut parents = vec![0; graph.len()];
    let mut stack = vec![start];

    while let Some(node) = stack.pop() {
        for &adj in graph[node as usize].iter() {
            if parents[adj as usize] != 0 {
                continue;
            }

            parents[adj as usize] = node;
            stack.push(adj);
        }
    }

    parents
}
