enum RGB {
    R,
    G,
    B,
}

use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut input = || input.next().unwrap();

    let n = input() as usize;
    let costs: Vec<_> = (0..n).map(|_| [input(), input(), input()]).collect();
    let mut min_costs = vec![[0; 3]; n];
    min_costs[0] = costs[0];

    let (r, g, b) = (RGB::R as usize, RGB::G as usize, RGB::B as usize);

    for i in 1..costs.len() {
        min_costs[i] = [
            costs[i][r] + min_costs[i - 1][g].min(min_costs[i - 1][b]),
            costs[i][g] + min_costs[i - 1][r].min(min_costs[i - 1][b]),
            costs[i][b] + min_costs[i - 1][r].min(min_costs[i - 1][g]),
        ];
    }
    // println!("{min_costs:?}");
    println!("{}", min_costs[n - 1].iter().min().unwrap());
}
