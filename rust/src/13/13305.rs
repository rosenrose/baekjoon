use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i64>);

    let n = input.next().unwrap() as usize;
    let distances: Vec<_> = input.by_ref().take(n - 1).collect();
    let costs: Vec<_> = input.collect();

    let mut min_cost = 0;
    let mut cursor = 0;
    let mut cur_cost = costs[cursor];

    while let Some(mut next_cheaper) = costs[cursor..].iter().position(|&cost| cost < cur_cost) {
        next_cheaper += cursor;
        min_cost += cur_cost * distances[cursor..next_cheaper].iter().sum::<i64>();
        // println!("{cursor} {next_cheaper}");
        cursor = next_cheaper;
        cur_cost = costs[cursor];
    }

    min_cost += cur_cost * distances[cursor..].iter().sum::<i64>();
    println!("{min_cost}");
}
