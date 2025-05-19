enum RGB {
    R = 0,
    G,
    B,
}

use std::io;

const MAX: usize = 1000;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let n = input.next().unwrap() as usize;
    let mut costs = [[0; 3]; MAX];

    for i in 0..n {
        costs[i] = [(); 3].map(|_| input.next().unwrap());
    }

    let mut min_costs = [[0; 3]; MAX];
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
