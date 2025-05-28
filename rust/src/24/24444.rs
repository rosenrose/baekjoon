use std::collections::VecDeque;
use std::fmt::Write;
use std::io;

const MAX: usize = 100_000 + 1;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let mut output = String::new();

    let [n, m, r] = [(); 3].map(|_| input.next().unwrap());
    let mut adjacency_list = [(); MAX].map(|_| Vec::new());

    for [u, v] in (0..m).map(|_| [(); 2].map(|_| input.next().unwrap())) {
        adjacency_list[u].push(v as i32);
        adjacency_list[v].push(u as i32);
    }

    for list in &mut adjacency_list[..=n] {
        (*list).sort_unstable();
    }

    let visited = bfs(&adjacency_list[..=n], r);

    for order in &visited[1..=n] {
        writeln!(output, "{order}").unwrap();
    }

    print!("{output}");
}

fn bfs(graph: &[Vec<i32>], start: usize) -> [i32; MAX] {
    let mut visited = [0; MAX];
    visited[start] = 1;

    let mut count = 2;
    let mut queue = VecDeque::from([start as i32]);

    while let Some(node) = queue.pop_front() {
        for &adj in &graph[node as usize] {
            if visited[adj as usize] != 0 {
                continue;
            }

            visited[adj as usize] = count;
            count += 1;
            queue.push_back(adj);
        }
    }

    visited
}
