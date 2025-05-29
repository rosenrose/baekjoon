use std::io;

const MAX: usize = 100_000;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let mut input = || input.next().unwrap();

    let n = input();
    let mut adjacency_array = ([i32::MAX; MAX], [(0, 0); MAX - 1]);

    for (i, [p, c]) in (0..n - 1).map(|_| [(); 2].map(|_| input())).enumerate() {
        let prev = adjacency_array.0[p];

        adjacency_array.0[p] = i as i32;
        adjacency_array.1[i] = (c as i32, prev);
    }

    let mut costs = [[0; 2]; MAX];

    for (i, cost) in (0..n).map(|_| [(); 2].map(|_| input() as i32)).enumerate() {
        costs[i] = cost;
    }

    let white_first_cost = dfs(&adjacency_array, &costs[..n], 0);
    let black_first_cost = dfs(&adjacency_array, &costs[..n], 1);

    println!("{}", white_first_cost.min(black_first_cost));
}

fn dfs((nodes, edges): &([i32; MAX], [(i32, i32); MAX - 1]), costs: &[[i32; 2]], first: u8) -> i64 {
    let mut cost = 0;
    let mut stack = vec![(0, first)];

    while let Some((node, color)) = stack.pop() {
        cost += costs[node as usize][color as usize] as i64;

        let adj_color = (color == 0) as u8;
        let mut edge = nodes[node as usize];

        while let Some(&(adj, next_edge)) = edges.get(edge as usize) {
            stack.push((adj, adj_color));
            edge = next_edge;
        }
    }

    cost
}
