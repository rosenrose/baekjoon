use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut input = || input.next().unwrap();

    let (n, m) = (input() as usize, input());
    let mut adjacency_list = vec![Vec::new(); n + 1];

    for (u, v) in (0..m).map(|_| (input(), input())) {
        adjacency_list[u as usize].push(v);
        adjacency_list[v as usize].push(u);
    }

    println!("{}", dfs(&adjacency_list));
}

fn dfs(graph: &[Vec<i32>]) -> i32 {
    let mut count = 0;
    let mut visited = vec![false; graph.len()];

    for start in 1..graph.len() {
        if visited[start] {
            continue;
        }

        visited[start] = true;
        let mut stack = vec![start as i32];

        while let Some(node) = stack.pop() {
            for &adj in graph[node as usize].iter() {
                if visited[adj as usize] {
                    continue;
                }

                visited[adj as usize] = true;
                stack.push(adj);
            }
        }

        count += 1;
    }

    count
}
