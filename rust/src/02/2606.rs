use std::io;

const MAX: usize = 100 + 1;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let [_, edges_cnt] = [(); 2].map(|_| input.next().unwrap());
    let mut adjacency_list = [(); MAX].map(|_| Vec::new());

    for [u, v] in (0..edges_cnt).map(|_| [(); 2].map(|_| input.next().unwrap())) {
        adjacency_list[u].push(v);
        adjacency_list[v].push(u);
    }

    println!("{}", dfs(&adjacency_list, 1));
}

fn dfs(graph: &[Vec<usize>], start: usize) -> i32 {
    let mut visited = [false; MAX];
    let mut count = 0;
    let mut stack = vec![start];

    visited[start] = true;

    while let Some(node) = stack.pop() {
        count += 1;

        for &adj in &graph[node] {
            if visited[adj] {
                continue;
            }

            visited[adj] = true;
            stack.push(adj);
        }
    }

    count - 1
}
