use std::io;

const MAX: usize = 100;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.lines().flat_map(str::parse::<usize>);

    let n = input.next().unwrap();
    let mut graph = [0; MAX];

    for (i, num) in input.enumerate() {
        graph[i] = num - 1;
    }

    let (cycle_nodes, cycle_nodes_len) = get_cycle_nodes(&graph, n);

    println!("{}", cycle_nodes_len);

    for num in &cycle_nodes[..cycle_nodes_len] {
        println!("{}", num + 1);
    }
}

fn get_cycle_nodes(graph: &[usize], graph_len: usize) -> ([usize; MAX], usize) {
    let mut cycle_nodes = [0; MAX];
    let mut cycle_nodes_len = 0;
    let mut visited_total = [false; MAX];
    let mut visited_dfs = [false; MAX];

    for start in 0..graph_len {
        if visited_total[start] {
            continue;
        }

        dfs(
            start,
            &graph,
            &mut visited_total,
            &mut visited_dfs,
            &mut cycle_nodes,
            &mut cycle_nodes_len,
        );
    }

    cycle_nodes[..cycle_nodes_len].sort();

    (cycle_nodes, cycle_nodes_len)
}

fn dfs(
    node: usize,
    graph: &[usize],
    visited_total: &mut [bool],
    visited_dfs: &mut [bool],
    cycle_nodes: &mut [usize],
    cycle_nodes_len: &mut usize,
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

        dfs(
            adj,
            graph,
            visited_total,
            visited_dfs,
            cycle_nodes,
            cycle_nodes_len,
        )
    };

    visited_dfs[node] = false;

    if let Some(end) = cycle_end_node {
        cycle_nodes[*cycle_nodes_len] = node;
        *cycle_nodes_len += 1;

        if end == node {
            return None;
        }
    }

    cycle_end_node
}
