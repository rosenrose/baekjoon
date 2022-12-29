use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<usize>().unwrap());
    let mut input = || input.next().unwrap();

    let (nodes, edges) = (input(), input());

    let adjacency_list = (0..edges).fold(vec![Vec::new(); nodes + 1], |mut acc, _| {
        let (u, v) = (input(), input());
        acc[u].push(v);
        acc[v].push(u);

        acc
    });

    dfs(&adjacency_list, 1);
}

fn dfs(graph: &Vec<Vec<usize>>, start: usize) {
    let mut discovered = vec![false; graph.len()];
    discovered[start] = true;

    let mut stack = vec![start];
    let mut count = 0;

    while let Some(node) = stack.pop() {
        for &neighbor in graph[node].iter() {
            if discovered[neighbor] {
                continue;
            }

            discovered[neighbor] = true;
            stack.push(neighbor);
            count += 1;
        }
    }

    println!("{count}");
}
