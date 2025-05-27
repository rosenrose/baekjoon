use std::io;

const MAX: usize = 100_000 + 1;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let n = input.next().unwrap() as usize;
    let mut adjacency_list = [(); MAX].map(|_| Vec::new());

    for [a, b] in (0..n - 1).map(|_| [(); 2].map(|_| input.next().unwrap())) {
        adjacency_list[a as usize].push(b);
        adjacency_list[b as usize].push(a);
    }

    for list in &mut adjacency_list[..=n] {
        (*list).sort_unstable();
    }

    println!("{}", dfs_judge(&adjacency_list[..=n], input) as u8);
}

fn dfs_judge(graph: &[Vec<i32>], mut path: impl Iterator<Item = i32>) -> bool {
    let start = path.next().unwrap();

    if start != 1 {
        return false;
    }

    let mut visited = [false; MAX];

    dfs(graph, start, &mut visited, path.by_ref())
}

fn dfs(
    graph: &[Vec<i32>],
    node: i32,
    visited: &mut [bool],
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
