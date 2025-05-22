use std::fmt::Write;
use std::io;

const MAX: usize = 100_000 + 1;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let mut output = String::new();

    for _ in 0..input.next().unwrap() {
        let n = input.next().unwrap();
        let mut adjacency_list = [0; MAX];

        for (i, num) in input.by_ref().take(n).enumerate() {
            adjacency_list[i + 1] = num;
        }

        let count = get_cycle_nodes(&adjacency_list[..=n]);

        writeln!(output, "{count}").unwrap();
    }

    print!("{output}");
}

fn get_cycle_nodes(graph: &[usize]) -> usize {
    let graph_len = graph.len();
    let mut visited_total = [false; MAX];
    let mut visited_dfs = [false; MAX];
    let mut count = graph_len - 1;

    for start in 1..graph_len {
        if visited_total[start] {
            continue;
        }

        dfs(
            start,
            &graph,
            &mut visited_total[..graph_len],
            &mut visited_dfs[..graph_len],
            &mut count,
        );
    }

    count
}

fn dfs(
    node: usize,
    graph: &[usize],
    visited_total: &mut [bool],
    visited_dfs: &mut [bool],
    count: &mut usize,
) -> Option<usize> {
    visited_total[node] = true;
    visited_dfs[node] = true;

    let adj = graph[node];
    let cycle_end_node = 'a: {
        if visited_dfs[adj] {
            break 'a Some(adj);
        }

        if visited_total[adj] {
            break 'a None;
        }

        dfs(adj, graph, visited_total, visited_dfs, count)
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
