use std::collections::VecDeque;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let mut input = || input.next().unwrap();

    let n = input();
    let mut adjacency_list = vec![Vec::new(); n + 1];

    for (a, b) in (0..n).map(|_| (input(), input())) {
        adjacency_list[a].push(b);
        adjacency_list[b].push(a);
    }

    let cycle_nodes = get_cycle_nodes(&adjacency_list);
    let start = cycle_nodes.iter().position(|&b| b).unwrap();

    let mut dists = vec![0; adjacency_list.len()];
    let mut visited = vec![false; adjacency_list.len()];
    visited[start] = true;

    let mut queue = VecDeque::from([(start, 0)]);

    while let Some((node, dist)) = queue.pop_front() {
        dists[node] = dist;

        for &adj in &adjacency_list[node] {
            if visited[adj] {
                continue;
            }

            visited[adj] = true;

            if cycle_nodes[adj] {
                queue.push_back((adj, 0));
            } else {
                queue.push_back((adj, dist + 1));
            }
        }
    }

    for dist in &dists[1..] {
        print!("{dist} ");
    }
}

fn get_cycle_nodes(graph: &[Vec<usize>]) -> Vec<bool> {
    let mut cycle_nodes = vec![false; graph.len()];
    let start = 1;

    let mut visited = vec![None; graph.len()];
    visited[start] = Some(start);

    dfs(start, &graph, &mut cycle_nodes, &mut visited);

    cycle_nodes
}

fn dfs(
    node: usize,
    graph: &[Vec<usize>],
    cycle_nodes: &mut Vec<bool>,
    visited: &mut Vec<Option<usize>>,
) -> Option<usize> {
    let parent = visited[node].unwrap();
    let mut cycle_end_node = None;

    for &adj in &graph[node] {
        if adj == parent || cycle_nodes[adj] {
            continue;
        }

        if visited[adj].is_some() {
            cycle_end_node = Some(adj);
            continue;
        }

        visited[adj] = Some(node);
        cycle_end_node = dfs(adj, graph, cycle_nodes, visited).or(cycle_end_node);
    }

    if let Some(end) = cycle_end_node {
        cycle_nodes[node] = true;

        if end == node {
            return None;
        }
    }

    cycle_end_node
}
