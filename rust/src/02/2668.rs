use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let input = buf.lines().flat_map(str::parse::<usize>);

    let graph: Vec<_> = input.skip(1).map(|num| num - 1).collect();
    let cycle_nodes = get_cycle_nodes(&graph);

    println!("{}", cycle_nodes.len());

    for num in cycle_nodes {
        println!("{}", num + 1);
    }
}

fn get_cycle_nodes(graph: &[usize]) -> Vec<usize> {
    let mut cycle_nodes = Vec::new();
    let mut visited_total = vec![false; graph.len()];
    let mut visited_dfs = vec![false; graph.len()];

    for start in 0..graph.len() {
        if visited_total[start] {
            continue;
        }

        dfs(
            start,
            &graph,
            &mut visited_total,
            &mut visited_dfs,
            &mut cycle_nodes,
        );
    }

    cycle_nodes.sort();
    cycle_nodes
}

fn dfs(
    node: usize,
    graph: &[usize],
    visited_total: &mut Vec<bool>,
    visited_dfs: &mut Vec<bool>,
    cycle_nodes: &mut Vec<usize>,
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

        dfs(adj, graph, visited_total, visited_dfs, cycle_nodes)
    };

    visited_dfs[node] = false;

    if let Some(end) = cycle_end_node {
        cycle_nodes.push(node);

        if end == node {
            return None;
        }
    }

    cycle_end_node
}
