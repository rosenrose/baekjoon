use std::collections::VecDeque;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let mut input = || input.next().unwrap();

    let n = input();
    let (start, end) = (input(), input());
    let mut adjacency_list = vec![Vec::new(); n + 1];

    for (x, y) in (0..input()).map(|_| (input(), input())) {
        adjacency_list[x].push(y);
        adjacency_list[y].push(x);
    }

    let dist = bfs(&adjacency_list, start, end);

    println!("{dist}");
}

fn bfs(graph: &[Vec<usize>], start: usize, end: usize) -> i32 {
    let mut visited = vec![false; graph.len()];
    let mut queue = VecDeque::from([(start, 0)]);
    visited[start] = true;

    while let Some((node, dist)) = queue.pop_front() {
        if node == end {
            return dist;
        }

        for &adj in graph[node].iter() {
            if visited[adj] {
                continue;
            }

            visited[adj] = true;
            queue.push_back((adj, dist + 1));
        }
    }

    -1
}
