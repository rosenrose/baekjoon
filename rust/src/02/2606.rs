use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let mut input = || input.next().unwrap();

    let (nodes, edges) = (input(), input());
    let mut adjacency_list = vec![Vec::new(); nodes + 1];

    for (u, v) in (0..edges).map(|_| (input(), input())) {
        adjacency_list[u].push(v);
        adjacency_list[v].push(u);
    }

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
