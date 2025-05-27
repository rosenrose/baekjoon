use std::collections::VecDeque;
use std::io;

const MAX: usize = 3000 + 1;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let n = input.next().unwrap();
    let mut adjacency_list = [(); MAX].map(|_| Vec::new());

    for [a, b] in (0..n).map(|_| [(); 2].map(|_| input.next().unwrap())) {
        adjacency_list[a].push(b);
        adjacency_list[b].push(a);
    }

    let cycle_nodes = get_cycle_nodes(&adjacency_list[..=n]);
    let start = cycle_nodes[..=n].iter().position(|&b| b).unwrap();

    let mut dists = [0; MAX];
    let mut visited = [false; MAX];
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

    for dist in &dists[1..=n] {
        print!("{dist} ");
    }
}

fn get_cycle_nodes(graph: &[Vec<usize>]) -> [bool; MAX] {
    let mut cycle_nodes = [false; MAX];
    let start = 1;

    let mut visited = [None; MAX];
    visited[start] = Some(start);

    dfs(
        start,
        &graph,
        &mut cycle_nodes[..graph.len()],
        &mut visited[..graph.len()],
    );

    cycle_nodes
}

fn dfs(
    node: usize,
    graph: &[Vec<usize>],
    cycle_nodes: &mut [bool],
    visited: &mut [Option<usize>],
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
