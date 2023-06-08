use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let mut output = String::new();

    for _ in 0..input.next().unwrap() {
        let n = input.next().unwrap();
        let mut adjacency_list = vec![0; n + 1];

        for (i, num) in input.by_ref().take(n).enumerate() {
            adjacency_list[i + 1] = num;
        }

        let count = get_cycle_nodes(&adjacency_list);

        writeln!(output, "{count}").unwrap();
    }

    print!("{output}");
}

fn get_cycle_nodes(graph: &[usize]) -> usize {
    let mut visited = vec![false; graph.len()];
    let mut visited_dfs = vec![false; graph.len()];
    let mut count = graph.len() - 1;

    for start in 1..graph.len() {
        if visited[start] {
            continue;
        }

        dfs(start, &graph, &mut visited, &mut visited_dfs, &mut count);
    }

    count
}

fn dfs(
    node: usize,
    graph: &[usize],
    visited: &mut Vec<bool>,
    visited_dfs: &mut Vec<bool>,
    count: &mut usize,
) -> Option<usize> {
    visited[node] = true;
    visited_dfs[node] = true;

    let adj = graph[node];
    let cycle_end_node = 'a: {
        if visited_dfs[adj] {
            break 'a Some(adj);
        }

        if visited[adj] {
            break 'a None;
        }

        dfs(adj, graph, visited, visited_dfs, count)
    };

    visited_dfs[node] = false;

    if let Some(end) = cycle_end_node {
        *count -= 1;

        if end == node {
            return None;
        }
    }

    cycle_end_node
}
