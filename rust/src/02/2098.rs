use std::io;

const MAX: usize = 16;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_whitespace().flat_map(str::parse::<i32>);

    let n = input.next().unwrap() as usize;
    let mut adjacency_matrix = [[0; MAX]; MAX];

    for r in 0..n {
        for (c, num) in input.by_ref().take(n).enumerate() {
            adjacency_matrix[r][c] = if num == 0 { i32::MAX } else { num };
        }
    }

    let mut memo = [[None; 1 << MAX]; MAX];
    let min_cost = tsp(&adjacency_matrix, n, 0, 1, &mut memo);

    println!("{min_cost}");
}

fn tsp(
    graph: &[[i32; MAX]],
    graph_len: usize,
    node: usize,
    visited: usize,
    memo: &mut [[Option<i32>; 1 << MAX]],
) -> i32 {
    if let Some(cost) = memo[node][visited] {
        return cost;
    }
    if visited == (1 << graph_len) - 1 {
        return graph[node][0];
    }

    let result = (0..graph_len)
        .map(|next_node| {
            let bit = 1 << next_node;

            if visited & bit != 0 || graph[node][next_node] == i32::MAX {
                return i32::MAX;
            }

            graph[node][next_node].saturating_add(tsp(
                graph,
                graph_len,
                next_node,
                visited | bit,
                memo,
            ))
        })
        .min()
        .unwrap();

    memo[node][visited] = Some(result);
    result
}
