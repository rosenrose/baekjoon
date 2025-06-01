use std::io;

const NODES_MAX: usize = 100_000 + 1;
const EDGES_MAX: usize = (NODES_MAX - 1) * 2;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut input = || input.next().unwrap();

    let v = input() as usize;
    let mut adjacency_array = ([i32::MAX; NODES_MAX], [((0, 0), 0); EDGES_MAX]);
    let mut i = 0;

    for _ in 0..v {
        let a = input();

        while let b @ 1.. = input() {
            let c = input();

            let prev = adjacency_array.0[a as usize];
            adjacency_array.0[a as usize] = i as i32;
            adjacency_array.1[i] = ((b, c), prev);
            i += 1;
        }
    }

    let (diameter_start, _) = dfs(&adjacency_array, 1);
    let (_diameter_end, diameter) = dfs(&adjacency_array, diameter_start);

    println!("{diameter}");
}

fn dfs(
    (nodes, edges): &([i32; NODES_MAX], [((i32, i32), i32); EDGES_MAX]),
    start: i32,
) -> (i32, i32) {
    let (mut most_far_node, mut max_dist) = (0, 0);
    let mut visited = [false; NODES_MAX];
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
