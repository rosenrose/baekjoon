use std::io;

const MAX: usize = 1000 + 1;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let mut input = || input.next().unwrap();

    let [n, m] = [(); 2].map(|_| input());
    let mut adjacency_list = [(); MAX].map(|_| Vec::new());

    for [u, v] in (0..m).map(|_| [(); 2].map(|_| input() as i16)) {
        adjacency_list[u as usize].push(v);
        adjacency_list[v as usize].push(u);
    }

    println!("{}", dfs(&adjacency_list[..=n]));
}

fn dfs(graph: &[Vec<i16>]) -> i16 {
    let mut count = 0;
    let mut visited = [false; MAX];

    for start in 1..graph.len() {
        if visited[start] {
            continue;
        }

        visited[start] = true;
        let mut stack = vec![start as i16];

        while let Some(node) = stack.pop() {
            for &adj in &graph[node as usize] {
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
