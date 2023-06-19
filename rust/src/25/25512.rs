use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let mut input = || input.next().unwrap();

    let n = input();
    let mut adjacency_array = (vec![i32::MAX; n], Vec::with_capacity(n - 1));

    for (p, c) in (0..n - 1).map(|_| (input(), input())) {
        let prev = adjacency_array.0[p];

        adjacency_array.0[p] = adjacency_array.1.len() as i32;
        adjacency_array.1.push((c as i32, prev));
    }

    let costs: Vec<_> = (0..n).map(|_| [(); 2].map(|_| input() as i32)).collect();
    let white_first_cost = dfs(&adjacency_array, &costs, 0);
    let black_first_cost = dfs(&adjacency_array, &costs, 1);

    println!("{}", white_first_cost.min(black_first_cost));
}

fn dfs((nodes, edges): &(Vec<i32>, Vec<(i32, i32)>), costs: &[[i32; 2]], first: u8) -> i64 {
    let mut cost = 0;
    let mut stack = vec![(0, first)];

    while let Some((node, color)) = stack.pop() {
        cost += costs[node as usize][color as usize] as i64;

        let adj_color = u8::from(color == 0);
        let mut edge = nodes[node as usize];

        while let Some(&(adj, next_edge)) = edges.get(edge as usize) {
            stack.push((adj, adj_color));
            edge = next_edge;
        }
    }

    cost
}
