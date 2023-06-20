use std::collections::VecDeque;
use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let mut output = String::new();

    let [n, m, r] = [(); 3].map(|_| input.next().unwrap());
    let mut adjacency_array = (vec![i32::MAX; n + 1], Vec::with_capacity(m * 2));

    for [u, v] in (0..m).map(|_| [(); 2].map(|_| input.next().unwrap())) {
        let prev = adjacency_array.0[u];
        adjacency_array.0[u] = adjacency_array.1.len() as i32;
        adjacency_array.1.push((v as i32, prev));

        let prev = adjacency_array.0[v];
        adjacency_array.0[v] = adjacency_array.1.len() as i32;
        adjacency_array.1.push((u as i32, prev));
    }

    let visited = bfs(&adjacency_array, r);

    for depth in &visited[1..] {
        writeln!(output, "{depth}").unwrap();
    }

    print!("{output}");
}

fn bfs((nodes, edges): &(Vec<i32>, Vec<(i32, i32)>), start: usize) -> Vec<i32> {
    let mut visited = vec![-1; nodes.len()];
    visited[start] = 0;

    let mut queue = VecDeque::from([(start as i32, 0)]);

    while let Some((node, depth)) = queue.pop_front() {
        let next_depth = depth + 1;
        let mut edge = nodes[node as usize];

        while let Some(&(adj, next_edge)) = edges.get(edge as usize) {
            if visited[adj as usize] == -1 {
                visited[adj as usize] = next_depth;
                queue.push_back((adj, next_depth));
            }

            edge = next_edge;
        }
    }

    visited
}
