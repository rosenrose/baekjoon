use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let mut input = || input.next().unwrap();
    let mut output = String::new();

    let n = input();
    let mut adjacency_list = vec![Vec::new(); n + 1];

    for (a, b) in (0..n - 1).map(|_| (input(), input())) {
        adjacency_list[a].push(b);
        adjacency_list[b].push(a);
    }

    let mut depths = vec![0; n + 1];
    let mut parents = vec![0; n + 1];
    let mut stack = vec![(1, 1)];

    while let Some((node, depth)) = stack.pop() {
        depths[node] = depth;

        for &adj in &adjacency_list[node] {
            if depths[adj] != 0 {
                continue;
            }

            parents[adj] = node;
            stack.push((adj, depth + 1));
        }
    }

    for (u, v) in (0..input()).map(|_| (input(), input())) {
        writeln!(output, "{}", lca((u, v), &depths, &parents)).unwrap();
    }

    print!("{output}");
}

fn lca((mut u, mut v): (usize, usize), depths: &[i32], parents: &[usize]) -> usize {
    while depths[u] > depths[v] {
        u = parents[u];
    }
    while depths[v] > depths[u] {
        v = parents[v];
    }

    while u != v {
        u = parents[u];
        v = parents[v];
    }

    u
}
