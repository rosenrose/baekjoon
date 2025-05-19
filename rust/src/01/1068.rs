use std::io;

const MAX: usize = 50;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let _n = input.next().unwrap() as usize;
    let delete_node = input.next_back().unwrap() as usize;
    let mut root = 0;
    let mut adjacency_list = [(); MAX].map(|_| Vec::new());

    for (node, parent) in input.enumerate() {
        if parent == -1 {
            root = node;
            continue;
        }
        if node == delete_node || parent as usize == delete_node {
            continue;
        }

        adjacency_list[parent as usize].push(node);
    }

    if root == delete_node {
        println!("0");
        return;
    }

    println!("{}", dfs(&adjacency_list, root));
}

fn dfs(graph: &[Vec<usize>], node: usize) -> i32 {
    if graph[node].is_empty() {
        return 1;
    }

    graph[node].iter().map(|&adj| dfs(graph, adj)).sum()
}
