use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut input = || input.next().unwrap();

    let n = input() as usize;
    let mut adjacency_list = vec![Vec::new(); n + 1];

    for (a, b) in (0..n - 1).map(|_| (input(), input())) {
        adjacency_list[a as usize].push(b);
        adjacency_list[b as usize].push(a);
    }

    for list in &mut adjacency_list {
        (*list).sort_unstable();
    }

    let path = (0..n).map(|_| input());

    println!("{}", u8::from(dfs_judge(&adjacency_list, path)));
}

fn dfs_judge(graph: &[Vec<i32>], mut path: impl Iterator<Item = i32>) -> bool {
    let start = path.next().unwrap();

    if start != 1 {
        return false;
    }

    let mut visited = vec![false; graph.len()];

    dfs(graph, start, &mut visited, path.by_ref())
}

fn dfs(
    graph: &[Vec<i32>],
    node: i32,
    visited: &mut Vec<bool>,
    path: &mut impl Iterator<Item = i32>,
) -> bool {
    visited[node as usize] = true;

    let adjacents = &graph[node as usize];
    let count = adjacents
        .iter()
        .filter(|&&adj| !visited[adj as usize])
        .count();

    for _ in 0..count {
        let next_node = path.next().unwrap();

        if adjacents.binary_search(&next_node).is_err() {
            return false;
        }

        let is_right_order = dfs(graph, next_node, visited, path);

        if !is_right_order {
            return false;
        }
    }

    true
}
