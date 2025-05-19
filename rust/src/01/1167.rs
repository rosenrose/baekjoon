use std::io;

const MAX: usize = 100_000 + 1;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut input = || input.next().unwrap();

    let v = input() as usize;
    let mut adjacency_array = ([i32::MAX; MAX], Vec::with_capacity(v));

    for _ in 0..v {
        let a = input();

        while let b @ 1.. = input() {
            let c = input();

            let prev = adjacency_array.0[a as usize];
            adjacency_array.0[a as usize] = adjacency_array.1.len() as i32;
            adjacency_array.1.push(((b, c), prev));

            let prev = adjacency_array.0[b as usize];
            adjacency_array.0[b as usize] = adjacency_array.1.len() as i32;
            adjacency_array.1.push(((a, c), prev));
        }
    }

    let (diameter_start, _) = dfs(&adjacency_array, 1);
    let (_diameter_end, diameter) = dfs(&adjacency_array, diameter_start);

    println!("{diameter}");
}

fn dfs((nodes, edges): &([i32; MAX], Vec<((i32, i32), i32)>), start: i32) -> (i32, i32) {
    let (mut most_far_node, mut max_dist) = (0, 0);
    let mut visited = [false; MAX];
    visited[start as usize] = true;

    let mut stack = vec![(start, 0)];

    while let Some((node, dist)) = stack.pop() {
        let mut edge = nodes[node as usize];

        if dist > max_dist {
            (most_far_node, max_dist) = (node, dist);
        }

        while let Some(&((adj, weight), next_edge)) = edges.get(edge as usize) {
            if !visited[adj as usize] {
                visited[adj as usize] = true;
                stack.push((adj, dist + weight));
            }

            edge = next_edge;
        }
    }

    (most_far_node, max_dist)
}
