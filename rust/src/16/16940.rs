use std::collections::VecDeque;
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

    println!("{}", u8::from(bfs(&adjacency_list, path)));
}

fn bfs(graph: &[Vec<i32>], mut path: impl Iterator<Item = i32>) -> bool {
    let start = path.next().unwrap();

    if start != 1 {
        return false;
    }

    let mut visited = vec![false; graph.len()];
    visited[start as usize] = true;

    let mut queue = VecDeque::from([start]);

    while let Some(node) = queue.pop_front() {
        let adjacents = &graph[node as usize];

        for &adj in adjacents {
            if visited[adj as usize] {
                continue;
            }

            visited[adj as usize] = true;
            let next_node = path.next().unwrap();

            if adjacents.binary_search(&next_node).is_err() {
                return false;
            }

            queue.push_back(next_node);
        }
    }

    true
}