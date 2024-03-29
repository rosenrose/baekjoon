use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let mut output = String::new();

    let n = input.next().unwrap();
    let mut adjacency_array = (vec![i32::MAX; n + 1], Vec::with_capacity((n - 1) * 2));

    for [a, b] in (0..n - 1).map(|_| [(); 2].map(|_| input.next().unwrap())) {
        let prev = adjacency_array.0[a];
        adjacency_array.0[a] = adjacency_array.1.len() as i32;
        adjacency_array.1.push((b as i32, prev));

        let prev = adjacency_array.0[b];
        adjacency_array.0[b] = adjacency_array.1.len() as i32;
        adjacency_array.1.push((a as i32, prev));
    }

    let parents = dfs(&adjacency_array, 1);

    for parent in &parents[2..] {
        writeln!(output, "{parent}").unwrap();
    }

    print!("{output}");
}

fn dfs((nodes, edges): &(Vec<i32>, Vec<(i32, i32)>), start: i32) -> Vec<i32> {
    let mut parents = vec![0; nodes.len()];
    let mut stack = vec![start];

    while let Some(node) = stack.pop() {
        let mut edge = nodes[node as usize];

        while let Some(&(adj, next_edge)) = edges.get(edge as usize) {
            if parents[adj as usize] == 0 {
                parents[adj as usize] = node;
                stack.push(adj);
            }

            edge = next_edge;
        }
    }

    parents
}
