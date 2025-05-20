use std::io;

const MAX: usize = 10000 + 1;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let n = input.next().unwrap();
    let mut adjacency_list = [(); MAX].map(|_| Vec::new());

    for [a, b, c] in (0..n - 1).map(|_| [(); 3].map(|_| input.next().unwrap())) {
        adjacency_list[a].push((b, c as i32));
        adjacency_list[b].push((a, c as i32));
    }

    let (diameter_start, _) = dfs(&adjacency_list, 1);
    let (_diameter_end, diameter) = dfs(&adjacency_list, diameter_start);

    println!("{diameter}");
}

fn dfs(graph: &[Vec<(usize, i32)>], start: usize) -> (usize, i32) {
    let (mut most_far_node, mut max_dist) = (0, 0);
    let mut visited = [false; MAX];
    visited[start] = true;

    let mut stack = vec![(start, 0)];

    while let Some((node, dist)) = stack.pop() {
        if dist > max_dist {
            (most_far_node, max_dist) = (node, dist);
        }

        for &(adj, weight) in &graph[node] {
            if visited[adj] {
                continue;
            }

            visited[adj] = true;
            stack.push((adj, dist + weight));
        }
    }

    (most_far_node, max_dist)
}
